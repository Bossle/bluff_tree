use crate::common::lp_solver::{DefaultSolver, Solver};

use super::*;
use std::collections::HashMap;

struct PlayerTree<S: Solver, P: PlayerTraits> {
    children: HashMap<(Vec<P::Message>, usize), (Vec<PlayerTree<S, P>>, S::Variable)>,
    strategy: S::Variable,
    weighted_value: S::Variable,
    end_corresps: Vec<(f64, S::Variable)>,
    temp_end: Vec<(f64, S::Variable)>,
}

impl<S: Solver, P: PlayerTraits> PlayerTree<S, P> {
    fn new(s_strat: &mut S, s_value: &mut S) -> PlayerTree<S, P> {
        PlayerTree {
            children: HashMap::new(),
            strategy: s_strat.new_var(),
            weighted_value: s_value.new_var(),
            end_corresps: Vec::new(),
            temp_end: Vec::new(),
        }
    }

    fn get_children(&mut self, m: Vec<P::Message>, c: usize, s_strat: &mut S, s_value: &mut S, direction: f64) -> &mut (Vec<PlayerTree<S, P>>, S::Variable) {
        self.children.entry((m, c)).or_insert_with(|| {
            let mut children: Vec<PlayerTree<S, P>> = std::iter::repeat_with(|| PlayerTree::new(s_strat, s_value)).take(c).collect();
            let root_val = s_value.new_var();
            let mut sum_vec = vec![(-1., self.strategy.clone())];
            for child in &mut children {
                s_value.add_constraint(&vec![(direction, root_val.clone()), (-direction, child.weighted_value.clone())], Ordering::Greater, 0.);
                sum_vec.push((1., child.strategy.clone()));
            }
            s_strat.add_constraint(&sum_vec, Ordering::Equal, 0.);
            (children, root_val)
        })
    }
}

pub fn solve<G: Game + Clone>(root: &mut Tree<G>) {
    let mut s1 = DefaultSolver::new();
    let mut s2 = DefaultSolver::new();
    let mut p1 = PlayerTree::new(&mut s2, &mut s1);
    let mut p2 = PlayerTree::new(&mut s1, &mut s2);
    explore_rec(&mut s1, &mut s2, &mut p1, &mut p2, &Some(root), 1.0, &Vec::new(), &Vec::new());
    s1.add_constraint(&vec![(1., p1.strategy)], Ordering::Equal, 1.);
    s2.add_constraint(&vec![(1., p2.strategy)], Ordering::Equal, 1.);
    add_leaf_constraints(&mut s1, &p2);
    add_leaf_constraints(&mut s2, &p1);
    let sol1 = s1.solve(vec![(1., p2.weighted_value)]);
    let sol2 = s2.solve(vec![(-1., p1.weighted_value)]);
    extract_solution_rec::<DefaultSolver, G>(&p1, &p2, Some(root), &Vec::new(), &Vec::new(), &sol1, &sol2);
}

fn explore_rec<S: Solver, G: Game + Clone>(s1: &mut S, s2: &mut S, p1: &mut PlayerTree<S, G::P1>, p2: &mut PlayerTree<S, G::P2>, maybe_node: &Option<&Tree<G>>, nature: f64, msgs1: &Vec<<G::P1 as PlayerTraits>::Message>, msgs2: &Vec<<G::P2 as PlayerTraits>::Message>) {
    match maybe_node {
        None => {
            p1.temp_end.push((nature*-1., p2.strategy.clone()));
            p2.temp_end.push((nature*1., p1.strategy.clone()));
        }
        Some(node) => match node.node_type.clone() {
            NodeType::Message1(m) => {
                let mut next_msgs1 = msgs1.clone();
                next_msgs1.push(m);
                explore_rec(s1, s2, p1, p2, &node.children[0].as_ref(), nature, &next_msgs1, msgs2);
            }
            NodeType::Message2(m) => {
                let mut next_msgs2 = msgs2.clone();
                next_msgs2.push(m);
                explore_rec(s1, s2, p1, p2, &node.children[0].as_ref(), nature, msgs1, &next_msgs2);
            }
            NodeType::Player1(c) => {
                let children = &mut p1.get_children(msgs1.clone(), c.len(), s1, s2, 1.).0;
                for i in 0..c.len() {
                    explore_rec(s1, s2, &mut children[i], p2, &node.children[i].as_ref(), nature, &Vec::new(), msgs2);
                }
            }
            NodeType::Player2(c) => {
                let children = &mut p2.get_children(msgs2.clone(), c.len(), s2, s1, -1.).0;
                for i in 0..c.len() {
                    explore_rec(s1, s2, p1, &mut children[i], &node.children[i].as_ref(), nature, msgs1, &Vec::new());
                }
            }
            NodeType::Random(r) => {
                for i in 0..r.len() {
                    explore_rec(s1, s2, p1, p2, &node.children[i].as_ref(), nature*node.prob.as_ref().expect("random nodes should have prob")[i], msgs1, msgs2);
                }
            }
            NodeType::End => {
                p1.end_corresps.push((nature*node.value.expect("end nodes should have value"), p2.strategy.clone()));
                p2.end_corresps.push((nature*node.value.expect("end nodes should have value"), p1.strategy.clone()));
            },
        }
    };
}

fn add_leaf_constraints<S: Solver, T: PlayerTraits>(s: &mut S, p: &PlayerTree<S, T>) {
    let mut sum_vec = vec![(-1., p.weighted_value.clone())];
    for (_, c) in &p.children {
        sum_vec.push((1., c.1.clone()));
    }
    for e in &p.end_corresps {
        sum_vec.push(e.clone());
    }
    for t in &p.temp_end {
        sum_vec.push(t.clone());
    }
    s.add_constraint(&sum_vec, Ordering::Equal, 0.);
    for (_, c) in &p.children {
        for child in &c.0 {
            add_leaf_constraints(s, child);
        }
    }
}

fn extract_solution_rec<S: Solver, G: Game + Clone>(p1: &PlayerTree<S, G::P1>, p2: &PlayerTree<S, G::P2>, maybe_node: Option<&mut Tree<G>>, msgs1: &Vec<<G::P1 as PlayerTraits>::Message>, msgs2: &Vec<<G::P2 as PlayerTraits>::Message>, sol1: &impl Fn(S::Variable) -> f64, sol2: &impl Fn(S::Variable) -> f64) {
    match maybe_node {
        None => {
            return;
        }
        Some(node) => {
            let prob: Vec<f64>;
            match node.node_type.clone() {
                NodeType::Message1(m) => {
                    prob = vec![(1.)];
                    let mut next_msgs1 = msgs1.clone();
                    next_msgs1.push(m);
                    extract_solution_rec(p1, p2, node.children[0].as_mut(), &next_msgs1, msgs2, sol1, sol2);
                }
                NodeType::Message2(m) => {
                    prob = vec![(1.)];
                    let mut next_msgs2 = msgs2.clone();
                    next_msgs2.push(m);
                    extract_solution_rec(p1, p2, node.children[0].as_mut(), msgs1, &next_msgs2, sol1, sol2);
                }
                NodeType::Player1(c) => {
                    let children = &p1.children.get(&(msgs1.clone(), c.len())).expect("tree should be fully explored").0;
                    let root_prob = sol1(p1.strategy.clone());
                    prob = children.iter().map(|n| sol1(n.strategy.clone())/root_prob).collect();
                    if root_prob > EPS {
                        for i in 0..c.len() {
                            extract_solution_rec(&children[i], p2, node.children[i].as_mut(), &Vec::new(), msgs2, sol1, sol2);
                        }
                    }
                }
                NodeType::Player2(c) => {
                    let children = &p2.children.get(&(msgs2.clone(), c.len())).expect("tree should be fully explored").0;
                    let root_prob = sol2(p2.strategy.clone());
                    prob = children.iter().map(|n| sol2(n.strategy.clone())/root_prob).collect();
                    if root_prob > EPS {
                        for i in 0..c.len() {
                            extract_solution_rec(p1, &children[i], node.children[i].as_mut(), msgs1, &Vec::new(), sol1, sol2);
                        }
                    }
                }
                NodeType::Random(r) => {
                    prob = node.prob.clone().expect("random nodes should always have prob");
                    for i in 0..r.len() {
                        extract_solution_rec(p1, p2, node.children[i].as_mut(), msgs1, msgs2, sol1, sol2);
                    }
                }
                NodeType::End => {
                    prob = vec![];
                }
            }
            let mut value = 0.;
            for i in 0..node.children.len() {
                if let Some(v) = node.children[i].as_ref().and_then(|n| n.value) {
                    value += v*prob[i]
                }
            }
            node.prob = Some(prob);
            node.value = Some(value);
        }
    }
}
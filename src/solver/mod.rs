use crate::common::*;

mod explorer;
pub use explorer::{expand, expand_full};

mod exact;

#[derive(Debug, Clone)]
pub enum NodeType<G: Game> {
    Message1(<G::P1 as PlayerTraits>::Message),
    Message2(<G::P2 as PlayerTraits>::Message),
    Player1(Vec<<G::P1 as PlayerTraits>::Choice>),
    Player2(Vec<<G::P2 as PlayerTraits>::Choice>),
    Random(Vec<G::RandomChoice>),
    End,
}

#[derive(Debug, Clone)]
pub struct Tree<G: Game> {
    node_type: NodeType<G>,
    children: Vec<Option<Tree<G>>>,
    path: (G, Vec<usize>),
    value: Option<f64>,
    prob: Option<Vec<f64>>
}

impl<G: Game + Clone + Debug> Tree<G> {
    fn new_node(node_type: NodeType<G>, root: G, path: Vec<usize>, child_amt: usize) -> Tree<G> {
        Tree {
            node_type: node_type,
            children: vec_of_repeat(child_amt, None),
            path: (root, path),
            value: None,
            prob: None
        }
    }

    pub fn new(g: G) -> Tree<G> {
        let mut tree = explorer::make_node(g, vec![]);
        exact::solve(&mut tree);
        tree
    }
}

pub struct TreeGame<G: Game> {
    state: Tree<G>
}

impl<G: Game> TreeGame<G> {
    pub fn new(tree: Tree<G>) -> TreeGame<G> {
        TreeGame { state: tree }
    }
}

impl<G: Game + Clone> Game for TreeGame<G> {
    type P1 = G::P1;
    type P2 = G::P2;
    type RandomChoice = G::RandomChoice;
    fn step(&mut self, g: &mut dyn GameInterface<Self>) -> Option<()> {
        println!("Value: {:?}, Prob: {:?}", self.state.value, self.state.prob);
        let child = match self.state.node_type.clone() {
            NodeType::Message1(m) => {
                g.p1_message(&m)?;
                self.state.children[0].clone()
            }
            NodeType::Message2(m) => {
                g.p2_message(&m)?;
                self.state.children[0].clone()
            }
            NodeType::Player1(c) => {
                self.state.children[g.p1_choice(&c)?].clone()
            }
            NodeType::Player2(c) => {
                self.state.children[g.p2_choice(&c)?].clone()
            }
            NodeType::Random(v) => {
                self.state.children[g.random(&self.state.prob.clone()?, &v)?].clone()
            }
            NodeType::End => {
                g.end(self.state.value.expect("end node should have value"));
                return None;
            }
        };
        self.state = child.expect("tree should be fully expanded");
        Some(())
    }
}
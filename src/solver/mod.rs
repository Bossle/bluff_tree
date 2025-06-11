use crate::common::*;

mod explorer;
pub use explorer::{expand, expand_full};

#[derive(Debug, Clone)]
pub enum NodeType<G: Game> {
    Message1(<G::P1 as PlayerTraits>::Message),
    Message2(<G::P2 as PlayerTraits>::Message),
    Player1(Vec<<G::P1 as PlayerTraits>::Choice>),
    Player2(Vec<<G::P2 as PlayerTraits>::Choice>),
    Random(Vec<G::RandomChoice>, Vec<f64>),
    End,
}

#[derive(Debug, Clone)]
pub struct Tree<G: Game> {
    node_type: NodeType<G>,
    value: Option<f64>,
    children: Vec<Option<Tree<G>>>,
    path: (G, Vec<usize>)
}

impl<G: Game + Clone + Debug> Tree<G> {
    fn new_node(node_type: NodeType<G>, root: G, path: Vec<usize>, child_amt: usize) -> Tree<G> {
        Tree {
            node_type: node_type,
            value: None,
            children: vec_of_repeat(child_amt, None),
            path: (root, path),
        }
    }

    pub fn new(g: G) -> Tree<G> {
        explorer::make_node(g, vec![])
    }
}

pub struct TreeGame<G: Game> {
    state: Tree<G>
}

impl<G: Game + Clone> Game for TreeGame<G> {
    type P1 = G::P1;
    type P2 = G::P2;
    type RandomChoice = G::RandomChoice;
    fn step(&mut self, g: &mut dyn GameInterface<Self>) -> Option<()> {
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
            NodeType::Random(v, p) => {
                self.state.children[g.random(&p, &v)?].clone()
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
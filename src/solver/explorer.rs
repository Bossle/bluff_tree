use super::*;

#[derive(Debug)]
struct MockInterface<G: Game + Clone> {
    root: G,
    path: Vec<usize>,
    i: usize,
    recorded: Option<Tree<G>>,
}

impl<G: Game + Clone + Debug> GameInterface<G> for MockInterface<G> {
    fn random(&mut self, p: &Vec<f64>, v: &Vec<G::RandomChoice>) -> Option<usize> {
        let n = v.len();
        if let Some(choice) = self.path.get(self.i) {
            self.i += 1;
            Some(*choice)
        } else {
            assert!(self.recorded.is_none(), "recording should happen exactly once {:?}", self);
            let node_type = NodeType::Random(v.clone(), p.clone());
            let new_node = Tree::new_node(node_type, self.root.clone(), self.path.clone(), n);
            self.recorded = Some(new_node);
            None
        }
    }
    fn p1_choice(&mut self, v: &Vec<<G::P1 as PlayerTraits>::Choice>) -> Option<usize> {
        let n = v.len();
        if let Some(choice) = self.path.get(self.i) {
            self.i += 1;
            Some(*choice)
        } else {
            assert!(self.recorded.is_none(), "recording should happen exactly once {:?}", self);
            let node_type = NodeType::Player1(v.clone());
            let new_node = Tree::new_node(node_type, self.root.clone(), self.path.clone(), n);
            self.recorded = Some(new_node);
            None
        }
    }
    fn p2_choice(&mut self, v: &Vec<<G::P2 as PlayerTraits>::Choice>) -> Option<usize> {
        let n = v.len();
        if let Some(choice) = self.path.get(self.i) {
            self.i += 1;
            Some(*choice)
        } else {
            assert!(self.recorded.is_none(), "recording should happen exactly once {:?}", self);
            let node_type = NodeType::Player2(v.clone());
            let new_node = Tree::new_node(node_type, self.root.clone(), self.path.clone(), n);
            self.recorded = Some(new_node);
            None
        }
    }
    fn p1_message(&mut self, msg: &<G::P1 as PlayerTraits>::Message) -> Option<()> {
        if let Some(choice) = self.path.get(self.i) {
            self.i += 1;
            assert_eq!(0, *choice, "message should only have one choice {:?}", self);
            Some(())
        } else {
            assert!(self.recorded.is_none(), "recording should happen exactly once {:?}", self);
            let node_type = NodeType::Message1(msg.clone());
            let new_node = Tree::new_node(node_type, self.root.clone(), self.path.clone(), 1);
            self.recorded = Some(new_node);
            None
        }
    }
    fn p2_message(&mut self, msg: &<G::P2 as PlayerTraits>::Message) -> Option<()> {
        if let Some(choice) = self.path.get(self.i) {
            self.i += 1;
            assert_eq!(0, *choice, "message should only have one choice {:?}", self);
            Some(())
        } else {
            assert!(self.recorded.is_none(), "recording should happen exactly once {:?}", self);
            let node_type = NodeType::Message2(msg.clone());
            let new_node = Tree::new_node(node_type, self.root.clone(), self.path.clone(), 1);
            self.recorded = Some(new_node);
            None
        }
    }
    fn end(&mut self, value: f64) {
        assert_eq!(self.path.len(), self.i, "no choices should happen after end {:?}", self);
        assert!(self.recorded.is_none(), "recording should happen exactly once {:?}", self);
        let node_type = NodeType::End;
        let mut new_node = Tree::new_node(node_type, self.root.clone(), self.path.clone(), 0);
        new_node.value = Some(value);
        self.recorded = Some(new_node);
    }
}

pub fn make_node<G: Game + Clone + Debug>(mut root: G, path: Vec<usize>) -> Tree<G> {
    let mut mock = MockInterface {
        root: root.clone(),
        path: path,
        i: 0,
        recorded: None,
    };
    let mut game = root.clone();
    let mut i = 0;
    while  game.step(&mut mock).is_some() {
        root = game.clone();
        i = mock.i;
    }
    let mut ret = mock.recorded.expect("recording should happen exactly once");
    ret.path = (root, mock.path.split_at(i).1.to_vec());
    ret
}

pub fn expand<G: Game + Clone + Debug>(node: &mut Tree<G>, child: usize) {
    assert!(node.children[child].is_none(), "should not expand already expanded child");
    let mut child_path = node.path.1.clone();
    child_path.push(child);
    node.children[child] = Some(make_node(node.path.0.clone(), child_path));
}

pub fn expand_full<G: Game + Clone + Debug>(node: &mut Tree<G>) {
    for i in 0..node.children.len() {
        expand(node, i);
        expand_full(node.children[i].as_mut().expect("child should be expanded after expand()"))
    }
}
pub mod lp_solver;

pub use std::marker::PhantomData;
pub use std::fmt::{Display, Debug};
pub use std::cmp::Ordering;

pub trait Serializable {
    fn kind_sizes() -> Vec<usize>;
    fn serialize(&self) -> (usize, Vec<i32>);
}

pub trait PlayerTraits {
    type Message: Display + Serializable + Clone + Debug;
    type Choice: Display + Serializable + Clone + Debug;
}

pub trait Game {
    type P1: PlayerTraits + Clone + Debug;
    type P2: PlayerTraits + Clone + Debug;
    type RandomChoice: Display + Serializable + Clone + Debug;
    fn step(&mut self, _: &mut dyn GameInterface<Self>) -> Option<()>;
}

pub trait GameInterface<G: Game> {
    fn random(&mut self, p: &Vec<f64>, v: &Vec<G::RandomChoice>) -> Option<usize>;
    fn p1_choice(&mut self, v: &Vec<<G::P1 as PlayerTraits>::Choice>) -> Option<usize>;
    fn p2_choice(&mut self, v: &Vec<<G::P2 as PlayerTraits>::Choice>) -> Option<usize>;
    fn p1_message(&mut self, msg: &<G::P1 as PlayerTraits>::Message) -> Option<()>;
    fn p2_message(&mut self, msg: &<G::P2 as PlayerTraits>::Message) -> Option<()>;
    fn end(&mut self, value: f64);
}

pub trait Player<T: PlayerTraits> {
    fn receive_message(&mut self, msg: &T::Message);
    fn choose(&mut self, v: &Vec<T::Choice>) -> usize;
}

pub struct DefaultGameInterface<G: Game, R: FnMut(&Vec<f64>, &Vec<G::RandomChoice>)->usize, P1: Player<G::P1>, P2: Player<G::P2>, E: FnMut(f64)> {
    pub game_type: PhantomData<G>,
    pub randomer: R,
    pub player1: P1,
    pub player2: P2,
    pub ender: E,
}

impl<G: Game, R: FnMut(&Vec<f64>, &Vec<G::RandomChoice>)->usize, P1: Player<G::P1>, P2: Player<G::P2>, E: FnMut(f64)> GameInterface<G> for DefaultGameInterface<G, R, P1, P2, E> {
    fn random(&mut self, p: &Vec<f64>, v: &Vec<G::RandomChoice>) -> Option<usize> {
        Some((self.randomer)(p, v))
    }
    fn p1_choice(&mut self, v: &Vec<<G::P1 as PlayerTraits>::Choice>) -> Option<usize> {
        Some(self.player1.choose(v))
    }
    fn p2_choice(&mut self, v: &Vec<<G::P2 as PlayerTraits>::Choice>) -> Option<usize> {
        Some(self.player2.choose(v))
    }
    fn p1_message(&mut self, msg: &<G::P1 as PlayerTraits>::Message) -> Option<()> {
        self.player1.receive_message(msg);
        Some(())
    }
    fn p2_message(&mut self, msg: &<G::P2 as PlayerTraits>::Message) -> Option<()> {
        self.player2.receive_message(msg);
        Some(())
    }
    fn end(&mut self, value: f64) {
        (self.ender)(value)
    }
}

pub fn run_game<G: Game>(game: &mut G, g: &mut dyn GameInterface<G>) {
    loop {
        if game.step(g).is_none() {
            return;
        }
    }
}

pub fn vec_of_repeat<T: Clone>(n: usize, v: T) -> Vec<T> {
    std::iter::repeat_with(|| v.clone()).take(n).collect()
}
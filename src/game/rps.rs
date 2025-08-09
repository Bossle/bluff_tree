use crate::common::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct RPS {
    p1_choice: Option<i32>
}

impl RPS {
    pub fn new() -> RPS {
        RPS {
            p1_choice: None
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Choice {
    v: i32,
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.v)
    }
}

impl Serializable for Choice {
    fn kind_sizes() -> Vec<usize> {
        vec![3]
    }
    fn serialize(&self) -> (usize, Vec<i32>) {
        let mut v: Vec<i32> = vec_of_repeat(3, 0);
        v[(self.v-1) as usize] = 1;
        (0, v)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Traits {}

impl PlayerTraits for Traits {
    type Message = Choice;
    type Choice = Choice;
}

impl Game for RPS {
    fn step(&mut self, g: &mut dyn GameInterface<Self>) -> Option<()> {
        match self.p1_choice {
            None => {
                self.p1_choice = Some(g.p1_choice(&vec![Choice{v:1}, Choice{v:2}, Choice{v:3}])? as i32);
            }
            Some(p1) => {
                let p2 = g.p2_choice(&vec![Choice{v:1}, Choice{v:2}, Choice{v:3}])? as i32;
                let ans = (p1+4-p2)%3-1;
                g.end(ans as f64);
                return None;
            }
        }
        Some(())
    }
    type P1 = Traits;
    type P2 = Traits;
    type RandomChoice = Choice;
}
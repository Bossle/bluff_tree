use crate::common::*;
use good_lp::solvers::coin_cbc::CoinCbcProblem;
use good_lp::{self, coin_cbc, variable, Solution, SolverModel, Variable};

pub trait Solver {
    type Checkpoint;
    type Variable: Clone;
    fn new(var_capacity: usize) -> Self;
    fn new_var(&mut self) -> Self::Variable;
    fn save_checkpoint(&self) -> Self::Checkpoint;
    fn restore_checkpoint(&mut self, checkpoint: Self::Checkpoint);
    fn add_constraint(&mut self, v: Vec<(f64, Self::Variable)>, o: Ordering, c: f64);
    fn solve(&mut self, maximize_coeffs: Vec<(f64, Self::Variable)>) -> impl Fn(Variable) -> f64;
}

pub struct DefaultSolver {
    lp: CoinCbcProblem,
    vars: Vec<Variable>,
    goal: Variable,
    next_var: usize,
}

impl Solver for DefaultSolver {
    type Checkpoint = CoinCbcProblem;
    type Variable = good_lp::Variable;

    fn new(var_capacity: usize) -> Self {
        let mut problem = good_lp::variables!();
        let goal = problem.add(variable());
        let vars = problem.add_vector(variable(), var_capacity);
        DefaultSolver {
            lp: problem.maximise(goal).using(coin_cbc),
            vars: vars,
            goal: goal,
            next_var: 0,
        }
    }

    fn new_var(&mut self) -> Self::Variable {
        self.next_var += 1;
        self.vars[self.next_var-1]
    }

    fn save_checkpoint(&self) -> Self::Checkpoint {
        self.lp.clone()
    }

    fn restore_checkpoint(&mut self, checkpoint: Self::Checkpoint) {
        self.lp = checkpoint;
    }

    fn add_constraint(&mut self, coeffs: Vec<(f64, Self::Variable)>, ord: Ordering, constant: f64) {
        let mut expr = good_lp::Expression::with_capacity(coeffs.len());
        for (c, v) in coeffs {
            expr = expr+c*v;
        }
        self.lp.add_constraint(match ord {
            Ordering::Equal => expr.eq(constant),
            Ordering::Greater => expr.geq(constant),
            Ordering::Less => expr.leq(constant),
        });
    }

    fn solve(&mut self, mut maximize_coeffs: Vec<(f64, Self::Variable)>) -> impl Fn(Variable) -> f64 {
        maximize_coeffs.push((-1.0, self.goal));
        self.add_constraint(maximize_coeffs, Ordering::Equal, 0.0);
        let solution = self.lp.clone().solve().expect("lp system should be solvable");
        move |v| solution.value(v)
    }
}
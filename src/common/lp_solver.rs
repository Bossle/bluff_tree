use crate::common::*;
use good_lp::{self, coin_cbc, variable, Solution, SolverModel};

pub trait Solver {
    // todo: tie the variable to the solver instance
    type Variable: Clone;
    fn new() -> Self;
    fn new_var(&mut self) -> Self::Variable;
    fn add_constraint(&mut self, v: &Vec<(f64, Self::Variable)>, o: Ordering, c: f64);
    fn solve(&mut self, maximize_coeffs: Vec<(f64, Self::Variable)>) -> impl Fn(Self::Variable) -> f64;
}

pub struct DefaultSolver {
    next_var: usize,
    constraints: Vec<(Vec<(f64, usize)>, Ordering, f64)>,
}

impl Solver for DefaultSolver {
    type Variable = usize;

    fn new() -> Self {
        DefaultSolver {
            next_var: 0,
            constraints: Vec::new(),
        }
    }

    fn new_var(&mut self) -> Self::Variable {
        let v = self.next_var;
        self.next_var += 1;
        return v;
    }

    fn add_constraint(&mut self, coeffs: &Vec<(f64, Self::Variable)>, ord: Ordering, constant: f64) {
        self.constraints.push((coeffs.clone(), ord, constant));
    }

    fn solve(&mut self, maximize_coeffs: Vec<(f64, Self::Variable)>) -> impl Fn(Self::Variable) -> f64 {
        let mut problem = good_lp::variables!();
        let vars = problem.add_vector(variable(), self.next_var);
        let mut goal = good_lp::Expression::with_capacity(maximize_coeffs.len());
        for (c, v) in maximize_coeffs {
            goal = goal+c*vars[v]
        }
        let mut lp = problem.maximise(goal).using(coin_cbc);
        for (coeffs, ord, constant) in &self.constraints {
            let mut expr = good_lp::Expression::with_capacity(coeffs.len());
            for (c, v) in coeffs {
                expr = expr+*c*vars[*v];
            }
            lp.add_constraint(match ord {
                Ordering::Equal => expr.eq(*constant),
                Ordering::Greater => expr.geq(*constant),
                Ordering::Less => expr.leq(*constant),
            });
        }
        let solution = lp.clone().solve().expect("lp system should be solvable");
        move |v| solution.value(vars[v])
    }
}
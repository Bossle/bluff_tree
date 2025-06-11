use crate::common::*;
use std::fmt::Display;
use std::io;
use rand::Rng;

pub struct ConsolePlayer {
    pub prefix: String
}

fn print_vec_with_indices<T: Display>(v: &Vec<T>) {
    for i in 0..v.len() {
        print!("{i}: {}  ", v[i]);
    }
    println!();
}

impl<T: PlayerTraits> Player<T> for ConsolePlayer {
    fn choose(&mut self, v: &Vec<T::Choice>) -> usize {
        println!("{} choice:", self.prefix);
        print_vec_with_indices(&v);
        loop {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).expect("STDIN should choose");
            match buf.trim().parse::<usize>() {
                Ok(choice) => { return choice }
                Err(err) => { println!("Invalid choice {err}") }
            }
        }
    }
    fn receive_message(&mut self, msg: &T::Message) {
        println!("To {}:\n{}", self.prefix, msg.to_string());
    }
}

pub fn console_random<T: Display + Clone>(v: &Vec<T>) -> usize {
    println!("Random:");
    print_vec_with_indices(&v);
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("STDIN should choose");
        match buf.trim().parse::<usize>() {
            Ok(choice) => { return choice }
            Err(err) => { println!("Invalid choice {err}") }
        }
    }
}

pub fn rng_random<T: Clone>(p: &Vec<f64>, _: &Vec<T>) -> usize {
    let mut x = rand::thread_rng().gen_range(0.0..1.0);
    for i in 0..p.len() {
        x -= p[i];
        if x < 0.0 {
            return i;
        }
    }
    panic!("distribution should sum to 1")
}
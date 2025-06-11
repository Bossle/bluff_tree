use bluff_tree::game::tic_tac_toe::TicTacToe;
use bluff_tree::common::*;
use bluff_tree::cmd::*;

fn main() {
    run_game(&mut TicTacToe::new(), &mut DefaultGameInterface{
        game_type: std::marker::PhantomData,
        randomer: rng_random,
        player1: ConsolePlayer{prefix: "P1".to_string()},
        player2: ConsolePlayer{prefix: "P2".to_string()},
        ender: |x:f64|println!("End: {x}"),
    });
}

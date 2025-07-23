use bluff_tree::game::tic_tac_toe::TicTacToe;
use bluff_tree::common::*;
use bluff_tree::cmd::*;

fn main() {
    let tree = bluff_tree::solver::Tree::new(TicTacToe::new());
    run_game(&mut bluff_tree::solver::TreeGame::new(tree), &mut DefaultGameInterface{
        game_type: std::marker::PhantomData,
        randomer: rng_random,
        player1: ConsolePlayer{prefix: "P1".to_string()},
        player2: ConsolePlayer{prefix: "P2".to_string()},
        ender: |x:f64|println!("End: {x}"),
    });
}

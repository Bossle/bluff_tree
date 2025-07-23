use crate::common::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct TicTacToe {
    board: [[i32; 3]; 3],
    curr_player: i32
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Choice {
    x: usize,
    y: usize,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            board: [[0,0,0],[0,0,0],[0,0,0]],
            curr_player: 1,
        }
    }

    fn is_win(cells: &[i32]) -> Option<i32> {
        let player = cells[0];
        if player == 0 {
            None
        } else if cells.iter().all(|&x| x == player) {
            Some(player)
        } else {
            None
        }
    }

    fn play(&mut self, c: Choice) -> Option<i32> {
        if self.board[c.x][c.y] != 0 {
            return Some(-self.curr_player)
        }
        self.board[c.x][c.y] = self.curr_player;
        self.curr_player = -self.curr_player;
        TicTacToe::is_win(&[self.board[c.x][0], self.board[c.x][1], self.board[c.x][2]]).or_else(||
        TicTacToe::is_win(&[self.board[0][c.y], self.board[1][c.y], self.board[2][c.y]]).or_else(||
        TicTacToe::is_win(&[self.board[0][0], self.board[1][1], self.board[2][2]]).or_else(||
        TicTacToe::is_win(&[self.board[0][2], self.board[1][1], self.board[2][0]]))))
    }

    fn board_string(&self) -> String {
        self.board.map(|v| v.map(|p| ["O"," ","X"][(p+1) as usize]).join("|")).join("\n-+-+-\n")
    }

    fn empty_cells(&self) -> Vec<Choice> {
        (0..3).map(
            |x| (0..3).map(|y| Choice{x,y})
                .filter(|c| self.board[c.x][c.y] == 0)
            .collect::<Vec<Choice>>()
        ).collect::<Vec<Vec<Choice>>>()
        .concat()
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Serializable for Choice {
    fn kind_sizes() -> Vec<usize> {
        vec![9]
    }
    fn serialize(&self) -> (usize, Vec<i32>) {
        let mut v: Vec<i32> = vec_of_repeat(9, 0);
        v[self.x*3+self.y] = 1;
        (0, v)
    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}'s turn\n{}", self.curr_player, self.board_string())
    }
}

impl Serializable for TicTacToe {
    fn kind_sizes() -> Vec<usize> {
        vec![10]
    }

    fn serialize(&self) -> (usize, Vec<i32>) {
        let mut ret = self.board.concat();
        ret.push(self.curr_player);
        (0, ret)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Traits {}

impl PlayerTraits for Traits {
    type Message = TicTacToe;
    type Choice = Choice;
}

impl Game for TicTacToe {
    fn step(&mut self, g: &mut dyn GameInterface<Self>) -> Option<()> {
        let cells = self.empty_cells();
        match self.curr_player {
            1 => {
                let choice = g.p1_choice(&cells)?;
                if let Some(winner) = self.play(cells[choice]) {
                    assert_eq!(1, winner);
                    g.end(winner as f64);
                    return None
                }
                g.p2_message(&self)?;
            }
            -1 => {
                let choice = g.p2_choice(&cells)?;
                if let Some(winner) = self.play(cells[choice]) {
                    assert_eq!(-1, winner);
                    g.end(winner as f64);
                    return None
                }
                g.p1_message(&self)?;
            }
            _ => panic!("invalid curr_player")
        }
        if self.empty_cells().is_empty() {
            g.end(0.0);
            return None
        }
        Some(())
    }
    type P1 = Traits;
    type P2 = Traits;
    type RandomChoice = Choice;
}
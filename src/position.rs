use std::todo;

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Cell {
    PlayerOne,
    PlayerTwo,
    Empty,
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GameState {
    Playing,
    PlayerOneWin,
    PlayerTwoWin,
    Draw,
}

#[derive(Copy, Clone, Debug)]
pub enum CurrentTurn {
    PlayerOne,
    PlayerTwo,
}

#[derive(Clone)]
pub struct ArrayBoard {
    cells: [Cell; WIDTH * HEIGHT],
    heights: [usize; WIDTH],
    player_turn: CurrentTurn,
    game: String,
    num_moves: usize,
    state: GameState,
}

impl Default for ArrayBoard {
    fn default() -> Self {
        Self {
            cells: [Cell::Empty; WIDTH * HEIGHT],
            heights: [0; WIDTH],
            player_turn: CurrentTurn::PlayerOne,
            game: String::new(),
            num_moves: 0,
            state: GameState::Playing,
        }
    }
}

impl TryFrom<&str> for ArrayBoard {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut board = Self::default();

        for column_char in value.chars() {
            match column_char.to_digit(10) {
                Some(column) => {
                    board.play(column as usize)?;
                }
                _ => return Err("Could not parse {} as valid mode."),
            }
        }
        Ok(board)
    }
}

#[allow(unused)]
impl ArrayBoard {
    pub fn is_winning_move(&self, col: usize) -> bool {
        self.is_winning_vertical(col)
            || self.is_winning_vertical(col)
            || self.is_winning_diagonal(col)
    }

    pub fn is_winning_vertical(&self, col: usize) -> bool {
        todo!()
    }

    pub fn is_winning_horizontal(&self, col: usize) -> bool {
        // What I'll have to do here is check the whole row instead of just checking from the
        // placed piece.
        todo!()
    }

    pub fn is_winning_diagonal(&self, col: usize) -> bool {
        // What makes this very hard is that unlike the previous checks, this one isn't just based
        // around the last placed peice, as it can spawn from the middle.

        todo!()
    }

    pub fn play(&mut self, col: usize) -> Result<GameState, &str> {
        if let Some(err_message) = self.check_valid_move(col) {
            return Err(err_message);
        }

        todo!()
    }

    pub fn check_valid_move(&self, col: usize) -> Option<&str> {
        // check index

        if col > WIDTH {
            return Some("Column out of bounds");
        }

        // check if column is full

        let mut counter = col;
        loop {
            if self.cells[counter].is_empty() {
                break;
            }
            counter += HEIGHT;

            if counter > HEIGHT * WIDTH {
                return Some("Column full.");
            }
        }

        None
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn game(&self) -> String {
        self.game.to_owned()
    }

    pub fn player_turn(&self) -> CurrentTurn {
        self.player_turn
    }
}

const BOARD_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    // Access these variants using `Piece::X` or `Piece::O`
    X,
    O,
}

impl Piece {
    pub fn other(self) -> Piece{
        match self{
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

pub type Tile = Option<Piece>;
pub type Tiles = [[Tile;BOARD_SIZE]; BOARD_SIZE];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}

// This type represents the possible errors that can occur when making a move
#[derive(Debug, Clone)]
pub enum MoveError {
    /// The game was already over when a move was attempted
    GameAlreadyOver,

    /// The position provided was invalid
    InvalidPosition { row: usize, col: usize },

    /// The tile already contained another piece
    TileNotEmpty { other_piece: Piece, row: usize, col: usize },
}
#[derive(Debug, Clone)]
pub struct Game {
    tiles: Tiles,
    current_piece: Piece,
    winner: Option<Winner>,
}
impl Game {
    pub fn new() -> Self {
        Self {
            tiles: Default::default(),
            current_piece: Piece::X,
            winner: None,
        }
    }
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), MoveError>{
        if self.is_finished(){
            return Err(MoveError::GameAlreadyOver);
        }
        else if row >= self.tiles.len() || col >= self.tiles[0].len() {
            return Err(MoveError::InvalidPosition {row, col});
        }
        else if let Some(other_piece) = self.tiles[row][col] {
            return Err(MoveError::TileNotEmpty {other_piece, row, col});
        }
        self.tiles[row][col] = Some(self.current_piece);
        self.current_piece = self.current_piece.other();
        self.update_winner(row, col);
        // return our "nothing" value `()`
        Ok(())
    }

    fn update_winner(&mut self, row: usize, col: usize) {
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();

        let tiles_row = self.tiles[row];
        let tiles_col = [self.tiles[0][col], self.tiles[1][col], self.tiles[2][col]];

        assert!(rows == 3 && cols == 3,
        "This code was written with the assumption that there are three rows and columns");

        let tiles_diagonal_1 = if row == col {
            // Diagonal 1
        [self.tiles[0][0], self.tiles[1][1], self.tiles[2][2]]
        }
        else {
            // This will never produce a winner, so it is suitable to use for the case where the
            // last move isn't on diagonal 1 anyway.
            [None, None, None]
        };

        let tiles_diagonal_2 = if (rows - row - 1) == col {
            // Diagonal 2
        [self.tiles[0][2], self.tiles[1][1], self.tiles[2][0]]
        }
        else {
            // Our last move isn't on diagonal 2.
            [None, None, None]
        };

        fn check_winner(row: &[Tile]) -> Option<Winner> {
        // This is an "inner function". It is only visible to this update_winner method. We
            if row[0] == row[1] && row[1] == row[2] {
                match row[0] {
                    Some(Piece::X) => Some(Winner::X),
                    Some(Piece::O) => Some(Winner::O),
                    None => None,
                }
            }
            else {
                None
            }
        }
        self.winner = self.winner
            .or_else(|| check_winner(&tiles_row))
            .or_else(|| check_winner(&tiles_col))
            .or_else(|| check_winner(&tiles_diagonal_1))
            .or_else(|| check_winner(&tiles_diagonal_2));

        self.winner = self.winner.or_else(|| {
            if self.tiles.iter().all(|row| row.iter().all(|tile| tile.is_some())) {
                Some(Winner::Tie)
            }
            else {
                None
            }
        });
    }
    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }
    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }
    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }
    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_3_o_wins() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 2).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(0, 2).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn diag_1_x_wins() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn diag_2_x_wins() {
        let mut game = Game::new();
        game.make_move(0, 2).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(2, 0).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn row_2_o_wins() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(1, 0).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(1, 1).unwrap();
        game.make_move(0, 2).unwrap();
        game.make_move(1, 2).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn tie() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(0, 2).unwrap();
        game.make_move(2, 0).unwrap();
        game.make_move(2, 1).unwrap();
        game.make_move(2, 2).unwrap();
        game.make_move(1, 0).unwrap();
        game.make_move(1, 2).unwrap();
        game.make_move(1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }
}
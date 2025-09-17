/***
 * Example template for a chess engine.
 *
 * Course litterature.
 * Course: DD1337 Programming
 * KTH Royal Institute of Technology
 *
 * Author: Viola Söderlund <violaso@kth.se>
 * License: MIT
 * Latest change: 2025-09-12
 */

use std::{
    fmt::{self, write},
    io::empty,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    board: [[Piece; 8]; 8],
    active_color: Color,
    state: GameState,
    //...
}
pub struct Position {
    row: usize,
    column: usize,
}
impl Position{
    fn new(row: usize, column: usize) -> Position{
        Position {row, column};
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    kind: PieceKind,
    color: Color,
    //x: usize,
    //y: usize
}
impl Piece {
    fn new(kind: PieceKind, color: Color) -> Piece {
        Piece { kind, color }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
    None,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
    Pawn,
    Empty,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            board: Game::init_board(),
            active_color: Color::White,
            state: GameState::InProgress,
            //...
        }
    }

    fn init_board() -> [[Piece; 8]; 8] {
        let mut board: [[Piece; 8]; 8] = [[Piece::new(PieceKind::Empty, Color::None); 8]; 8];

        for row in 0..8 {
            for column in 0..8 {
                let color = match row {
                    0 | 1 => Color::White,
                    6 | 7 => Color::Black,
                    _ => Color::None,
                };
                if row == 0 || row == 7 {
                    match column {
                        0 | 7 => board[row][column] = Piece::new(PieceKind::Rook, color),
                        1 | 6 => board[row][column] = Piece::new(PieceKind::Knight, color),
                        2 | 5 => board[row][column] = Piece::new(PieceKind::Bishop, color),
                        3 => board[row][column] = Piece::new(PieceKind::King, color),
                        4 => board[row][column] = Piece::new(PieceKind::Queen, color),
                        _ => board[row][column] = Piece::new(PieceKind::Queen, color),
                    }
                } else if row == 1 || row == 6 {
                    board[row][column] = Piece::new(PieceKind::Pawn, color);
                }
            }
        }
        return board;
        /*   let mut origin_board: Vec<(Square, Piece)> = Vec::new();
        for i in 0..8{
                 for j in 0..8{
                     let square = Square::new(j,i);


                         let piece_kind =
                         if  square.y == 0 || square.y == 7{
                             if square.x == 0|| square.x == 7 {
                             PieceKind::Rook

                             } else if square.x == 1 || square.x == 6 {
                                 PieceKind::Knight

                             } else if square.x == 2 || square.x == 5 {
                                 PieceKind::Bishop

                             } else if square.x == 3 {
                                 PieceKind::King

                             } else {
                                 PieceKind::Queen
                             }
                         } else if square.y == 1 || square.y == 6  {
                             PieceKind::Pawn
                         }else{
                              PieceKind::empty;
                         };


                         let piece_color =
                         if square.y == 0 || square.y == 1 {
                                 Color::White
                         }else if square.y == 6 || square.y == 7{
                                 Color::Black
                         }else{
                             Color::None
                         };
                         let new_piece = Piece::new(piece_kind, piece_color);
                         origin_board.push((square, new_piece));
                      }
                 }
                 for i in 0..8 {
                     for j in 0..8  {
                     }
                 }*/
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &Position, _to: &Position) -> Option<GameState> {
        None
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &Position) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _position: &Position) -> Option<Vec<Position>> {
        let mut possible_moves: Vec<Position> = Vec::new();
        
        let current_board = &self.board;
        let mover_row = _position.row;
        let mover_column = _position.column;

        let piece_to_move = &current_board[mover_row][mover_column];

        match piece_to_move.kind {
            PieceKind::Rook => {
                for row in (0..mover_row).rev(){
                        let closest_piece = current_board[row][mover_column];
                          if closest_piece.color == Color::None {
                             possible_moves.push(Position::new(row, mover_column));
                             
                            }else if closest_piece.color != piece_to_move.color {
                                possible_moves.push(Position::new(row, mover_column));
                                break;
                            }else{
                                break;
                            }
                }
                    for row in mover_row+1..8{
                        let closest_piece = current_board[row][mover_column];

                        if closest_piece.color == Color::None {
                            possible_moves.push(Position::new(row, mover_column));
                            
                        }else if closest_piece.color != piece_to_move.color {
                            possible_moves.push(Position::new(row, mover_column));
                            break;

                        }else{
                            break;
                        }
                    }

                for column in (0..mover_column).rev() {
                    let closest_piece = current_board[mover_row][column];

                    if closest_piece.kind == PieceKind::Empty {
                        possible_moves.push(Position::new(mover_row, column));

                    }else if closest_piece.color != piece_to_move.color {
                        possible_moves.push(Position::new(mover_row, column));
                        break;

                    }else{
                        break;
                    }
                }
                for column in mover_column+1..8 {
                    let closest_piece = current_board[mover_row][column];
                   if closest_piece.kind == PieceKind::Empty {
                        possible_moves.push(Position::new(mover_row, column));

                    }else if closest_piece.color != piece_to_move.color {
                        possible_moves.push(Position::new(mover_row, column));
                        break;

                    }else{
                        break;
                    }
                }
            },

            PieceKind::Knight => ,
            PieceKind::Bishop => ,
            PieceKind::King => ,
            PieceKind::Queen => ,
            PieceKind::Pawn => ,
            PieceKind::Empty => ,
        

        None
         }
    }
}
/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        writeln!(f, "{:#?}", self.board)?;
        write!(f, "|:----------------------:|\n");
        for row in self.board {
            write!(f, "|");
            for piece in row {
                let piece_to_print = match piece.kind {
                    PieceKind::Rook => "R ",
                    PieceKind::Knight => "Kn",
                    PieceKind::Bishop => "B ",
                    PieceKind::King => "K ",
                    PieceKind::Queen => "Q ",
                    PieceKind::Pawn => "P ",
                    PieceKind::Empty => "* ",
                };

                write!(f, " {}", piece_to_print);
            }
            write!(f, "| \n")?;
            writeln!(f)?;
        }
        write!(f, "|:----------------------:|\n");
        Ok(())
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        println!("{:?}", game);

        // assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}

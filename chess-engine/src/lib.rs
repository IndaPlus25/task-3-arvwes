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

use std::{fmt, io::empty};

#[derive(Copy, Clone, Debug, PartialEq)]

pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    board: Vec<(Square, Piece)>,
    active_color: Color,
    state: GameState,
    //...
}
pub struct Square{
    x: usize,
    y: usize,
    //occupied: bool,
    //color: color
    
}
impl Square{
   pub fn new(x: usize, y: usize) -> Square{
        Square { 
            x,
            y,
         }
    }
}

pub struct Piece{
    kind: PieceKind, 
    color: Color,
    //x: usize,
    //y: usize
}
impl Piece{
    fn new(kind: PieceKind, color: Color) -> Piece{
        Piece { 
            kind,
            color,
         }
    }
    fn as_string(kind: PieceKind) -> String{
       match kind{
            PieceKind::Rook => "R",
            PieceKind::Knight => "Kn",
            PieceKind::Bishop => "B",
            PieceKind::King => "K",
            PieceKind::Queen => "Q",
            PieceKind::Pawn => "P",
            PieceKind::Empty => "*",
        }.to_string()
    }
}

pub enum Color{
    White,
    Black,
    None,
}

pub enum PieceKind{
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
            board: init_board(),
             active_color: Color::White,
            state: GameState::InProgress,
            //...
            
           
                }
            }
        
        fn init_board() -> Vec<(Square, Piece)> {
            let mut origin_board: Vec<(Square, Piece)> = Vec::new();
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
                    }
                        return origin_board;

                    
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
    pub fn get_possible_moves(&self, _postion: &Position) -> Option<Vec<String>> {
        None
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
        
        write!(f, "")
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

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
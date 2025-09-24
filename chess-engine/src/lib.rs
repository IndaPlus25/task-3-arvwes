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
#[derive(Debug, Clone, Copy, PartialEq)]
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
    ///Create the board, with starting pieces
    fn init_board() -> [[Piece; 8]; 8] {
        let mut board: [[Piece; 8]; 8] = [[Piece::new(PieceKind::Empty, Color::None); 8]; 8];

        //loops through every element in the array and creates a new piece with a type and collor
        for row in 0..8 {
            for column in 0..8 {
                // determans the color of the piece to  be added
                let color = match row {
                    0 | 1 => Color::White,
                    6 | 7 => Color::Black,
                    _ => Color::None,
                };
                //determens the type of peice
                if row == 0 || row == 7 {
                    match column {
                        0 | 7 => board[row][column] = Piece::new(PieceKind::Rook, color),
                        1 | 6 => board[row][column] = Piece::new(PieceKind::Knight, color),
                        2 | 5 => board[row][column] = Piece::new(PieceKind::Bishop, color),
                        3 => board[row][column] = Piece::new(PieceKind::King, color),
                        4 => board[row][column] = Piece::new(PieceKind::Queen, color),
                        _ => board[row][column] = Piece::new(PieceKind::Queen, color),
                    }
                    //places all the pawns
                } else if row == 1 || row == 6 {
                    board[row][column] = Piece::new(PieceKind::Pawn, color);
                }
            }
        }
        return board;
    }

    /// If the current game state is `InProgress` and the move is legal,
    /// move a piece and return the resulting state of the game.
         pub fn make_move(&mut self, from: &(usize, usize), to: &(usize, usize)) -> Option<GameState> {
            println!("in funciton ::{:?}", self.get_possible_moves(from));
            if self.get_possible_moves(from).is_some() && self.get_game_state() == GameState::InProgress {
                let possible_moves = self.get_possible_moves(from).unwrap();
                if possible_moves.contains(to) {
                    let board = &mut self.board;
                let from_row = from.0;
                let from_column = from.1;
                let to_row = to.0;
                let to_column = to.1;

                let piece_to_move = board[from_row][from_column];
                 board[to_row][to_column] = piece_to_move;
                 board[from_row][from_column] = Piece { kind: PieceKind::Empty, color: Color::None };
                let new_game_state = GameState::InProgress;
                
                if self.active_color == Color::Black{
                    self.active_color = Color::White;
                }else{
                    self.active_color = Color::Black;
                }
                return  Some(new_game_state);
                }else{
                    return None;
                }

                
            }else{
                return None;
            }

        
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &(usize, usize)) -> () {
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
    pub fn get_possible_moves(&self, positions: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
        const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];
        const ROOK_DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        const QUEEN_DIRECTIONS: [(i8, i8); 8] = [
            (-1, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];
        const KING_OFFSETS: [(i8, i8); 8] = QUEEN_DIRECTIONS;
        const KNIGHT_OFFSETS: [(i8, i8); 8] = [
            (-2, -1),
            (-2, 1),
            (-1, 2),
            (-1, -2),
            (2, -1),
            (2, 1),
            (1, -2),
            (-1, -2),
        ];
        const WHITE_PAWN_OFFSETS: [(i8, i8); 4] = [(1, 0), (1, -1), (1, 1), (2, 0)];

        let mut possible_moves: Vec<(usize, usize)> = Vec::new();

        let current_board: &[[Piece; 8]; 8] = &self.board;
        let current_row: usize = positions.0;
        let current_column: usize = positions.1;

        let piece_to_move: &Piece = &current_board[current_row][current_column];

        ///checks if the move in sequence is possible (used for pieces without dsitance move constraints),
        /// return true if the next move may be possible aswell,
        ///adds possible moves to the possible_moves vector
        fn is_move_ok(
            current_board: &[[Piece; 8]; 8],
            possible_moves: &mut Vec<(usize, usize)>,
            piece_to_move: &Piece,
            new_row: usize,
            new_column: usize,
        ) -> bool {
            let blocking_piece: &Piece = &current_board[new_row][new_column];
            if blocking_piece.kind == PieceKind::Empty {
                possible_moves.push((new_row, new_column));
                return true; //empty square keep checking if next square is empty
            } else if blocking_piece.color != piece_to_move.color {
                possible_moves.push((new_row, new_column));
                return false; //stops further moves
            } else {
                return false; //piece in same collor is blocking
            }
        }

        fn check_sequence_of_moves(
            _directions: &[(i8, i8)],
            current_row: usize,
            current_column: usize,
            current_board: &[[Piece; 8]; 8],
            piece_to_move: &Piece,
            possible_moves: &mut Vec<(usize, usize)>,
        ) {
            let directions = _directions;

            //loops through all directions
            for (row, column) in directions {
                let mut next = 1;
                //Loops through and adds possible moves in the chosen dierction to possible_moves
                // untill an impossible move is found
                loop {
                    let new_row: i8 = current_row as i8 + row * next;
                    let new_column: i8 = current_column as i8 + column * next;
                    if new_row < 0 || new_row > 7 || new_column < 0 || new_column > 7 {
                        break;
                    }

                    if !is_move_ok(
                        current_board,
                        possible_moves,
                        piece_to_move,
                        new_row as usize,
                        new_column as usize,
                    ) {
                        break;
                    }
                    next += 1;
                }
            }
        }

        match piece_to_move.kind {
            PieceKind::Rook => {
                check_sequence_of_moves(
                    &ROOK_DIRECTIONS,
                    current_row,
                    current_column,
                    current_board,
                    piece_to_move,
                    &mut possible_moves,
                );
                if possible_moves.is_empty() {
                    return None;
                }else{
                     return Some(possible_moves);
                }
            }
            PieceKind::Knight => {
                for (row_offset, column_offset) in KNIGHT_OFFSETS {
                    let new_row: i8 = current_row as i8 + row_offset;
                    let new_column: i8 = current_column as i8 + column_offset;
                    if !(new_row < 0 || new_row > 7 || new_column < 0 || new_column > 7) {
                        is_move_ok(
                            current_board,
                            &mut possible_moves,
                            piece_to_move,
                            new_row as usize,
                            new_column as usize,
                        );
                    }
                }
                if possible_moves.is_empty() {
                    return None;
                }else{
                     return Some(possible_moves);
                }
            }
            PieceKind::Bishop => {
                check_sequence_of_moves(
                    &BISHOP_DIRECTIONS,
                    current_row,
                    current_column,
                    current_board,
                    piece_to_move,
                    &mut possible_moves,
                );
                if possible_moves.is_empty() {
                    return None;
                }else{
                     return Some(possible_moves);
                }
            }
            //this is not finished
            PieceKind::King => {
                for (row_offset, column_offset) in KING_OFFSETS {
                    let new_row: i8 = current_row as i8 + row_offset;
                    let new_column: i8 = current_column as i8 + column_offset;
                    if !(new_row < 0 || new_row > 7 || new_column < 0 || new_column > 7) {
                        is_move_ok(
                            current_board,
                            &mut possible_moves,
                            piece_to_move,
                            new_row as usize,
                            new_column as usize,
                        );
                    }
                }
                if possible_moves.is_empty() {
                    return None;
                }else{
                     return Some(possible_moves);
                }
            }
            // check_sequence_of_moves(ROOK_DIRECTIONS),
            PieceKind::Queen => {
                check_sequence_of_moves(
                    &QUEEN_DIRECTIONS,
                    current_row,
                    current_column,
                    current_board,
                    piece_to_move,
                    &mut possible_moves,
                );
                if possible_moves.is_empty() {
                    return None;
                }else{
                     return Some(possible_moves);
                }
            }
            PieceKind::Pawn => {
             
                for (white_row_offset, column_offset) in WHITE_PAWN_OFFSETS {
                    let mut row_offset = white_row_offset;
                    if piece_to_move.color == Color::Black {
                         row_offset = -white_row_offset;
                    }
                    let new_row = (current_row as i8 + row_offset) as usize;
                    let new_column = (current_column as i8 + column_offset) as usize;
                       if !(new_row < 0 || new_row > 7 || new_column < 0 || new_column > 7) {
                    let blocking_piece: Piece = current_board[new_row][new_column];

                    if blocking_piece.color != piece_to_move.color
                        && blocking_piece.color != Color::None
                        && new_column != current_column
                    {
                        possible_moves.push((new_row, new_column));
                    } else if blocking_piece.color == Color::None && current_column == new_column {
                        possible_moves.push((new_row, new_column));
                    }
                }
                }
                if possible_moves.is_empty() {
                    return None;
                }else{
                     return Some(possible_moves);
                }

               
            }
            PieceKind::Empty => {
                return None;
            } 
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
        write!(f, " possible moves : {:?}", self.get_possible_moves(&(1, 1))).ok();

        /* build board representation string */
        // writeln!(f, "{:#?}", self.board)?;
        write!(f, "\n|:----------------------:|\n").ok();
        for row in self.board {
            write!(f, "|").ok();
            //
            //pairs a piece with it corresponding name and saves it to piece_as_string
            for piece in row {
                let mut piece_as_string: String = match piece.kind {
                    PieceKind::Rook => "R ",
                    PieceKind::Knight => "N ",
                    PieceKind::Bishop => "B ",
                    PieceKind::King => "K ",
                    PieceKind::Queen => "Q ",
                    PieceKind::Pawn => "P ",
                    PieceKind::Empty => "* ",
                }
                .to_string();
                //if the piece is black lower case is used
                if piece.color == Color::Black {
                    piece_as_string = piece_as_string.to_lowercase();
                }
                write!(f, " {}", piece_as_string).ok();
            }
            write!(f, "| \n")?;
            writeln!(f)?;
        }
        write!(f, "|:----------------------:|\n").ok();
        Ok(()) // i dont know what this does, but its needed
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
        let mut game = Game::new();
        game.make_move(&(1,1), &(3,1));
        game.make_move(&(6,2), &(4,2));
        game.make_move(&(3,1), &(4,2));
        game.make_move(&(0,2), &(2,0));
        println!("{:?}", game);

        // assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}

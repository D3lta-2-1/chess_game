use std::mem::swap;
use std::ops::Range;
use crate::game::piece_registry::MovementKind::*;
use crate::game::chess_board::Player::{Black, White};
use crate::game::grid::{Grid, BOARD_SIZE};
use crate::game::piece_registry::{Movement, PieceKind, PieceRegistry};
use crate::game::selection::Selection;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Player {
    Black,
    White
}

impl Player {
    fn flip(&mut self) {
        *self = match self {
            Black => White,
            White => Black
        }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Piece {
    pub player:  Player,
    pub piece_kind: PieceKind,
    pub not_moved: bool,
}

impl Piece {
    fn new(player:  Player, piece_kind: PieceKind) -> Option<Self> {
        Some(Self {
            player,
            piece_kind,
            not_moved: true,
        })
    }

    fn pawn_row(player: Player) -> [Option<Self>; 8] {
        [Self::new(player, PieceKind(0)); 8]
    }

    //todo: add Serde support to put this in a json file
    fn heavy_row(player: Player) -> [Option<Self>; 8] {
        [
            Self::new(player, PieceKind(1)),
            Self::new(player, PieceKind(2)),
            Self::new(player, PieceKind(3)),
            Self::new(player, PieceKind(4)),
            Self::new(player, PieceKind(5)),
            Self::new(player, PieceKind(3)),
            Self::new(player, PieceKind(2)),
            Self::new(player, PieceKind(1)),
        ]
    }
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    grid: Grid<Option<Piece>>,
    turn: Player,
}

impl ChessBoard {
    fn new_grid() -> Grid<Option<Piece>> {
        use Player::*;
        Grid::from([
            Piece::heavy_row(Black),
            Piece::pawn_row(Black),
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            Piece::pawn_row(White),
            Piece::heavy_row(White),
        ])
    }

    pub fn new() -> Self {
        Self {
            grid: Self::new_grid(),
            turn: White
        }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        self.grid.get(x, y).clone()
    }

    fn is_in_grid(x: i32, y: i32) -> bool {
        const RANGE: Range<i32> = 0..(BOARD_SIZE as i32);
        RANGE.contains(&x) && RANGE.contains(&y)
    }

    fn is_empty(&self, x: i32, y: i32) -> bool {
        Self::is_in_grid(x, y) && self.get_piece(x as usize, y as usize) == None
    }

    fn is_opponent(&self, player: Player,  x: i32, y: i32) -> bool {
        Self::is_in_grid(x, y) && self.get_piece(x as usize, y as usize).is_some_and(|piece| piece.player != player)
    }

    fn is_not_friend(&self, player: Player,  x: i32, y: i32) -> bool {
        Self::is_in_grid(x, y) && self.get_piece(x as usize, y as usize).is_none_or(|piece| piece.player != player)
    }

    pub fn possible_choice(&self, registry: &PieceRegistry, x: usize, y: usize) -> Option<Selection> {
        let piece = self.get_piece(x, y);
        let piece = piece?;
        if piece.player != self.turn {
            return None
        }

        let mut selection = Selection::new(x, y);
        let flip = match piece.player {
            Black => 1,
            White => -1,
        };

        let x = x as i32;
        let y = y as i32;

        for Movement(kind, dx, dy) in registry.get_movement(piece.piece_kind)? {
            let dy = dy * flip;
            match kind {
                Blocking => if self.is_empty(x + dx, y + dy) {
                    selection.choice.add(x + dx, y + dy)
                }
                FirstMove => if self.is_empty(x + dx, y + dy) && piece.not_moved {
                    selection.choice.add(x + dx, y + dy)
                }
                Eating => if self.is_opponent(piece.player, x + dx,y + dy) {
                    selection.choice.add(x + dx, y + dy)
                }
                NotFriend => if self.is_not_friend(piece.player, x + dx, y + dy) {
                    selection.choice.add(x + dx, y + dy)
                }
                Trailing => {
                    let mut i = 1;
                    while self.is_not_friend(piece.player, x + dx * i, y + dy * i) {
                        selection.choice.add(x + dx * i, y + dy * i);
                        if self.is_opponent(piece.player,x + dx * i, y + dy * i) { break; }
                        i += 1;
                    }
                }

            }
        }
        Some(selection)
    }



    pub fn move_piece(&mut self, from_x: usize, from_y: usize, x: usize, y: usize) -> Option<Piece> {
        let mut temp = None;
        swap(self.grid.get_mut(from_x,from_y), &mut temp);
        if let Some(piece) = &mut temp {
            piece.not_moved = false
        }
        swap(self.grid.get_mut(x,y), &mut temp);
        self.turn.flip();
        temp
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, Option<&Piece>)> {
        self.grid.iter().map(|(x, y, piece)| (x, y, piece.as_ref()))
    }
}
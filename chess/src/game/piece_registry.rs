use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use vello::Scene;
use vello_svg::render;
use crate::game::chess_board::Player;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum MovementKind {
    Blocking, //only available if the case is empty
    Eating, //only available if the contains an opponent
    NotFriend, //available if the case is empty or contains an opponent
    Trailing, //all multiples available, but stop after the first occupied case
    FirstMove, //like blocking only on the first move
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement(pub MovementKind, pub i32, pub i32);

#[derive(Serialize, Deserialize)]
struct Piece {
    name: String,
    movements: Vec<Movement>,
    black_sprite: PathBuf,
    white_sprite: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pieces: Vec<Piece>,
}

impl Config {
    /*fn template() -> Self {
        Self {
            pieces: vec![Piece {
                name: "Pawn".to_string(),
                movements: vec![
                    Movement(Blocking, 0, 1),
                    Movement(Eating, 1, 1),
                    Movement(Eating, -1, 1),
                    Movement(FirstMove, 0, 2),
                ],
                black_sprite: "assets/black_pawn.svg".to_string(),
                white_sprite: "assets/white_pawn.svg".to_string(),
            }],
        }
    }*/

    /*fn make_template() {
        let config = Config::template();
        let file = File::create("template.json").unwrap();
        serde_json::to_writer_pretty(file, &config).unwrap();
    }*/
}

struct PieceData {
    name: String,
    movements: Vec<Movement>,
    black_sprite: Scene,
    white_sprite: Scene,
}

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
pub struct PieceKind(pub u8); //TODO: make this field private

pub struct PieceRegistry {
    id_allocator: u8,
    pieces: HashMap<PieceKind, PieceData>, //could be a little more optimized with a Vec, but it's not a big deal
}

impl PieceRegistry {
    fn new() -> Self {
        Self {
            id_allocator: 0,
            pieces: HashMap::new(),
        }
    }

    pub fn fake_it() -> Self {
        let mut registry = Self::new();
        let pawn = registry.register_piece(PieceData {
            name: "Pawn".to_string(),
            movements: vec![
                Movement(MovementKind::Blocking, 0, 1),
                Movement(MovementKind::Eating, 1, 1),
                Movement(MovementKind::Eating, -1, 1),
                Movement(MovementKind::FirstMove, 0, 2),
            ],
            black_sprite: render(include_str!("../../assets/black_pawn.svg")).unwrap(),
            white_sprite: render(include_str!("../../assets/white_pawn.svg")).unwrap(),
        });
        let rook = registry.register_piece(PieceData {
            name: "Rook".to_string(),
            movements: vec![
                Movement(MovementKind::Trailing, 0, 1),
                Movement(MovementKind::Trailing, 0, -1),
                Movement(MovementKind::Trailing, 1, 0),
                Movement(MovementKind::Trailing, -1, 0),
            ],
            black_sprite: render(include_str!("../../assets/black_rook.svg")).unwrap(),
            white_sprite: render(include_str!("../../assets/white_rook.svg")).unwrap(),
        });
        let knight = registry.register_piece(PieceData {
            name: "Knight".to_string(),
            movements: vec![
                Movement(MovementKind::NotFriend, 1, 2),
                Movement(MovementKind::NotFriend, 2, 1),
                Movement(MovementKind::NotFriend, 1, -2),
                Movement(MovementKind::NotFriend, 2, -1),
                Movement(MovementKind::NotFriend, -1, 2),
                Movement(MovementKind::NotFriend, -2, 1),
                Movement(MovementKind::NotFriend, -1, -2),
                Movement(MovementKind::NotFriend, -2, -1),
            ],
            black_sprite: render(include_str!("../../assets/black_knight.svg")).unwrap(),
            white_sprite: render(include_str!("../../assets/white_knight.svg")).unwrap(),
        });
        let bishop = registry.register_piece(PieceData {
            name: "Bishop".to_string(),
            movements: vec![
                Movement(MovementKind::Trailing, 1, 1),
                Movement(MovementKind::Trailing, 1, -1),
                Movement(MovementKind::Trailing, -1, 1),
                Movement(MovementKind::Trailing, -1, -1),
            ],
            black_sprite: render(include_str!("../../assets/black_bishop.svg")).unwrap(),
            white_sprite: render(include_str!("../../assets/white_bishop.svg")).unwrap(),
        });
        let queen = registry.register_piece(PieceData {
            name: "Queen".to_string(),
            movements: vec![
                Movement(MovementKind::Trailing, 0, 1),
                Movement(MovementKind::Trailing, 0, -1),
                Movement(MovementKind::Trailing, 1, 0),
                Movement(MovementKind::Trailing, -1, 0),
                Movement(MovementKind::Trailing, 1, 1),
                Movement(MovementKind::Trailing, 1, -1),
                Movement(MovementKind::Trailing, -1, 1),
                Movement(MovementKind::Trailing, -1, -1),
            ],
            black_sprite: render(include_str!("../../assets/black_queen.svg")).unwrap(),
            white_sprite: render(include_str!("../../assets/white_queen.svg")).unwrap(),
        });
        let king = registry.register_piece(PieceData {
            name: "King".to_string(),
            movements: vec![
                Movement(MovementKind::Blocking, 0, 1),
                Movement(MovementKind::Blocking, 0, -1),
                Movement(MovementKind::Blocking, 1, 0),
                Movement(MovementKind::Blocking, -1, 0),
                Movement(MovementKind::Blocking, 1, 1),
                Movement(MovementKind::Blocking, 1, -1),
                Movement(MovementKind::Blocking, -1, 1),
                Movement(MovementKind::Blocking, -1, -1),
            ],
            black_sprite: render(include_str!("../../assets/black_king.svg")).unwrap(),
            white_sprite: render(include_str!("../../assets/white_king.svg")).unwrap(),
        });
        registry
    }


    pub fn load_from_config(path: &Path) -> Self {
        let config_path = path.join("chess_config.json");
        let file = File::open(config_path).unwrap();
        let config: Config = serde_json::from_reader(file).unwrap();

        let mut registry = Self::new();

        for piece in config.pieces {
            let black_path = path.join(piece.black_sprite);
            let white_path = path.join(piece.white_sprite);

            let mut string = String::new();
            File::open(black_path).unwrap().read_to_string(&mut string).unwrap();
            let black_sprite = render(&string).unwrap();
            string.clear();
            File::open(white_path).unwrap().read_to_string(&mut string).unwrap();
            let white_sprite = render(&string).unwrap();
            registry.register_piece(PieceData {
                name: piece.name,
                movements: piece.movements,
                black_sprite,
                white_sprite,
            });
        }
        registry
    }

    fn register_piece(&mut self, data: PieceData) -> PieceKind {
        let piece = PieceKind(self.id_allocator);
        self.id_allocator += 1;
        self.pieces.insert(piece, data);
        piece
    }

    pub fn get_movement(&self, piece: PieceKind) -> Option<&[Movement]> {
        self.pieces.get(&piece).map(|data| &data.movements[..])
    }

    pub fn get_sprite(&self, piece: PieceKind, player: Player) -> &Scene {
        let data = self.pieces.get(&piece).unwrap();
        match player {
            Player::Black => &data.black_sprite,
            Player::White => &data.white_sprite,
        }
    }
}
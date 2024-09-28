use vello::kurbo::{Affine, Circle, Rect, Stroke};
use vello::peniko::{Color, Fill};
use vello::Scene;
use crate::game::Selection;
use crate::game::chess_board::ChessBoard;
use crate::game::piece_registry::PieceRegistry;

pub struct BoardRenderer {
    board_scene: Scene,
}

impl BoardRenderer {
    pub fn new() -> Self {
        let tile_colors =  [Color::rgb(0.9, 0.9, 0.8), Color::rgb(0.2, 0.3, 0.5)];
        Self {
            board_scene: Self::create_board(tile_colors),
        }
    }


    pub const CELL_SIZE: f64 = 100.0;
    pub const BOARD_SIZE: f64 = Self::CELL_SIZE * 8.0;

    fn create_board(tile_colors: [Color; 2]) -> Scene {
        let mut scene = Scene::new();
        for y in 0..8 {
            for x in 0..8 {
                let color = tile_colors[ (x + y) % 2];
                let x = x as f64;
                let y = y as f64;
                let affine = Affine::translate((x * Self::CELL_SIZE, y * Self::CELL_SIZE));
                let rect = Rect::new(0.0, 0.0, Self::CELL_SIZE, Self::CELL_SIZE);
                scene.fill(Fill::NonZero, affine, color, None, &rect);
            }
        }
        scene
    }

    pub fn draw_board(&self, scene: &mut Scene) {
        scene.append(&self.board_scene, None)
    }

    pub fn draw_pieces(&self, board: &ChessBoard, registry: &PieceRegistry, scene: &mut Scene) {
        for (x, y, piece) in board.iter() {
            if let Some(piece) = piece {
                let x = x as f64;
                let y = y as f64;
                let affine = Affine::translate((x * Self::CELL_SIZE, y * Self::CELL_SIZE));
                scene.append(registry.get_sprite(piece.piece_kind, piece.player), Some(affine));
            }
        }
    }

    pub fn draw_selection(&self, selection: &Option<Selection>, scene: &mut Scene) {
        if let Some(selection) = selection {

            let stroke = Stroke::new(6.0);

            let color = Color::RED;
            let center = Self::CELL_SIZE /2.0;
            let radius = center - 3.0;
            let x = selection.x as f64;
            let y = selection.y as f64;
            let affine = Affine::translate((x * Self::CELL_SIZE, y * Self::CELL_SIZE));
            let circle = Circle::new((center, center), radius);
            scene.stroke(&stroke, affine, color, None, &circle);

            for (x, y, available) in selection.choice.iter() {
                let circle = Circle::new((50.0, 50.0), 15.0);
                let color = Color::LIME; // todo: use a light opacity

                if *available {
                    let x = x as f64;
                    let y = y as f64;

                    let affine = Affine::translate((x * Self::CELL_SIZE, y * Self::CELL_SIZE));
                    scene.fill(Fill::NonZero, affine, &color, None, &circle);
                }
            }
        }
    }
}


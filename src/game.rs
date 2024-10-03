use std::time::Duration;
use vello::kurbo::{Affine, Point, Rect, Vec2};
use vello::Scene;
use crate::app::LogicHandler;
use crate::game::board_renderer::BoardRenderer;
use crate::game::chess_board::ChessBoard;
use crate::game::piece_registry::PieceRegistry;
use crate::game::selection::Selection;
#[cfg(not(target_os = "android"))]
use std::path::Path;

mod board_renderer;
mod chess_board;
mod selection;
mod grid;
mod piece_registry;

pub struct ChessGame {
    registry: PieceRegistry,
    boards: Vec<ChessBoard>,
    renderer: BoardRenderer,
    selection: Option<Selection>,
    scene: Scene,
    transform: Affine,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            #[cfg(not(target_os = "android"))]
            registry: PieceRegistry::load_from_config(&Path::new("config")),
            #[cfg(target_os = "android")]
            registry: PieceRegistry::fake_it(),
            boards: vec![ChessBoard::new()],
            renderer: BoardRenderer::new(),
            selection: None,
            scene: Scene::new(),
            transform: Affine::IDENTITY,
        }
    }

    fn clicked_on_cell(&mut self, x: usize, y: usize) {
        match &self.selection {
            None => {
                self.selection = self.actual_board().possible_choice(&self.registry, x, y);
            }
            Some(selection) => {
                if selection.choice.is_available(x, y) {
                    let mut new_board = self.actual_board().clone();
                    new_board.move_piece(selection.x, selection.y, x, y);
                    self.boards.push(new_board);
                }
                self.selection = None;
            }
        }
    }

    fn actual_board(&self) -> &ChessBoard {
        self.boards.last().unwrap()
    }

    fn refresh(&mut self) {
        self.scene.reset();
        self.renderer.draw_board(&mut self.scene);
        self.renderer.draw_pieces(self.boards.last().unwrap(), &self.registry, &mut self.scene);
        self.renderer.draw_selection(&self.selection, &mut self.scene);
    }
}

impl LogicHandler for ChessGame {
    fn on_mouse_click(&mut self, x: f64, y: f64) {
        let point_on_screen = Point::new(x, y);
        let point_on_board = self.transform.inverse() * point_on_screen;
        let case = Affine::scale(BoardRenderer::CELL_SIZE).inverse() * point_on_board;
        let rect = Rect::new(0.0, 0.0, BoardRenderer::BOARD_SIZE, BoardRenderer::BOARD_SIZE);

        if rect.contains(point_on_board) {
            let x = case.x as usize;
            let y = case.y as usize;
            self.clicked_on_cell(x, y);
        } else {
            self.selection = None;
        }
    }

    fn on_exit_press(&mut self) {
        self.selection = None;
        if self.boards.len() > 1 {
            self.boards.pop();
        }
    }

    fn draw(&mut self, scene: &mut Scene, _duration: Duration) {
        self.refresh();
        scene.append(&self.scene, Some(self.transform));
    }

    fn surface_resize(&mut self, width: u32, height: u32) {
        let min = u32::min(width, height) as f64;
        let scale_factor = min / BoardRenderer::BOARD_SIZE;

        let mut vec = Vec2::new(width as f64, height as f64);
        vec -= Vec2::new(BoardRenderer::BOARD_SIZE * scale_factor, BoardRenderer::BOARD_SIZE * scale_factor);
        vec /= 2.0;
        self.transform = Affine::scale(scale_factor).then_translate(vec);
    }
}
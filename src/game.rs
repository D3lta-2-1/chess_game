use std::path::Path;
use std::time::Duration;
use vello::kurbo::Affine;
use vello::Scene;
use crate::app::LogicHandler;
use crate::game::board_renderer::BoardRenderer;
use crate::game::chess_board::ChessBoard;
use crate::game::grid::BOARD_SIZE;
use crate::game::piece_registry::PieceRegistry;
use crate::game::selection::Selection;


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
    scale_factor: f64,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            registry: PieceRegistry::load_from_config(&Path::new("config")),
            boards: vec![ChessBoard::new()],
            renderer: BoardRenderer::new(),
            selection: None,
            scene: Scene::new(),
            scale_factor: 0.75,
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
        let cell_size = 100.0 * self.scale_factor;
        let x = (x / cell_size) as usize;
        let y = (y / cell_size) as usize;
        if x < BOARD_SIZE && y < BOARD_SIZE {
            self.clicked_on_cell(x, y);
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
        let transform = Affine::scale(self.scale_factor);
        scene.append(&self.scene, Some(transform));


    }
}
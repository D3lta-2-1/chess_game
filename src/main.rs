use winit::event_loop::EventLoop;
use chess_game::app::SimpleVelloApp;
use chess_game::game::ChessGame;

#[cfg(not(target_os = "android"))]
fn main() {
    // Setup a bunch of state:
    let game_state = ChessGame::new();
    let mut app = SimpleVelloApp::new(game_state);

    // Create and run a winit event loop
    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
}
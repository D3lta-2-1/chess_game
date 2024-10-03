//#![feature(is_none_or)] //depend on rust version

mod app;
mod game;

use anyhow::Result;
use winit::event_loop::EventLoop;
#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid;
use crate::app::SimpleVelloApp;
use crate::game::ChessGame;

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;

    // Setup a bunch of state:
    let game_state = ChessGame::new();
    let mut app = SimpleVelloApp::new(game_state);

    // Create and run a winit event loop
    EventLoop::with_user_event().with_android_app(android_app).build().unwrap()
        .run_app(&mut app)
        .expect("Couldn't run event loop");
}

pub fn start() -> Result<()> {
    // Setup a bunch of state:
    let game_state = ChessGame::new();
    let mut app = SimpleVelloApp::new(game_state);

    // Create and run a winit event loop
    let event_loop = EventLoop::new()?;
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}
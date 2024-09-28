// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

mod app;
mod game;

use anyhow::Result;
use winit::event_loop::EventLoop;
use crate::app::SimpleVelloApp;
use crate::game::ChessGame;

fn main() -> Result<()> {
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

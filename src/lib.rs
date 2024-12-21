extern crate log;
pub mod app;
pub mod game;

#[cfg(target_os = "android")]
#[export_name = "android_main"]
pub fn main(android_app: winit::platform::android::activity::AndroidApp) {

    extern crate android_logger;
    use android_logger::FilterBuilder;
    use log::LevelFilter::Off;

    use log::LevelFilter;
    use android_logger::Config;
    use crate::app::SimpleVelloApp;
    use winit::event_loop::EventLoop;
    use crate::game::ChessGame;

    let filter = FilterBuilder::new().filter(Some("wgpu_core"), Off).build();

    android_logger::init_once(
        Config::default().with_filter(filter).with_max_level(LevelFilter::Trace),
    );

    use winit::platform::android::EventLoopBuilderExtAndroid;

    // Setup a bunch of state:
    let game_state = ChessGame::new();
    let mut app = SimpleVelloApp::new(game_state);

    // Create and run a winit event loop
    EventLoop::with_user_event().with_android_app(android_app).handle_volume_keys().build().unwrap()
        .run_app(&mut app)
        .expect("Couldn't run event loop");
}


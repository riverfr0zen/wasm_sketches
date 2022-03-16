use bevy::core::FixedTimestep;
#[cfg(feature = "framestats")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::Resource;
use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use bevy::render::renderer::RenderDevice;
#[cfg(target_arch = "wasm32")]
use bevy::window::WindowCreated;
use bevy_prototype_lyon::prelude::*;
use core::marker::Send;
use rand::prelude::thread_rng;
use rand::Rng;


const TARGET_RES_WIDTH: f32 = 3840.0;
// const TARGET_RES_HEIGHT: f32 = 2160.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const WINDOW_WIDTH_DEV: f32 = 1600.0;
const WINDOW_HEIGHT_DEV: f32 = 1600.0;
const CLEAR_COLOR: Color = Color::rgb(0.1, 0.11, 0.0);


#[derive(Debug)]
pub struct WindowSetup {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub position_x: f32,
    pub position_y: f32,
}


impl Default for WindowSetup {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self {
                title: "Untitled Sketch".to_string(),
                width: WINDOW_WIDTH_DEV,
                height: WINDOW_HEIGHT_DEV,
                position_x: TARGET_RES_WIDTH - WINDOW_WIDTH_DEV,
                position_y: 0.0,
            }
        } else {
            Self {
                title: "Untitled Sketch".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                position_x: TARGET_RES_WIDTH - WINDOW_WIDTH,
                position_y: 0.0,
            }
        }
    }
}

pub trait SketchCoreSettingsProvider: Resource {
    fn get_window_setup(&self) -> WindowSetup;
}


/// Create a base "sketch" Bevy app that provides:
///
/// * Window setup
/// * A way to provide app_globals by passing a Struct that implements SketchCoreSettingsProvider
/// * Browser window size matching for WASM targets
///
pub fn sketch_factory(app_globals: &'static dyn SketchCoreSettingsProvider) -> App {
    let mut app = App::new();
    let winsetup = WindowSetup::default();
    app.insert_resource(WindowDescriptor {
        title: winsetup.title,
        width: winsetup.width,
        height: winsetup.height,
        position: Some(Vec2::new(winsetup.position_x, winsetup.position_y)),
        #[cfg(target_arch = "wasm32")]
        canvas: Some("#window-matching-canvas".to_string()),
        ..Default::default()
    })
    .insert_resource(app_globals)
    .insert_resource(ClearColor(CLEAR_COLOR))
    .insert_resource(Msaa { samples: 4 });

    return app;
}

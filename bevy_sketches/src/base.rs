#[cfg(feature = "framestats")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
#[cfg(feature = "debuglog")]
use bevy::log::LogSettings;
use bevy::prelude::*;
#[cfg(feature = "debuglog")]
use bevy::utils::tracing::Level;
use bevy_web_extras::prelude::*;


/// Build on web_app to provide some additional logging stuff
pub fn sketch(webcfg: WebExtrasCfg) -> App {
    let mut app = web_app(webcfg);

    #[cfg(feature = "debuglog")]
    app.insert_resource(LogSettings {
        level: Level::DEBUG,
        filter: "wgpu=error,bevy_render=error".to_string(),
    });
    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins);
    debug!("SKETCH: debug log level enabled");
    info!("SKETCH: info log level enabled");

    // Example of "feature-flipping".
    // See https://doc.rust-lang.org/cargo/reference/features.html
    #[cfg(feature = "framestats")]
    app.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    return app;
}

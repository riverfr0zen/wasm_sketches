use bevy::prelude::*;
use bevy::window::WindowCreated;
use bevy::core::FixedTimestep;
#[cfg(feature = "framestats")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;
use rand::prelude::thread_rng;


// pub const WINDOW_WIDTH: f32 = 3200.0;
// pub const WINDOW_HEIGHT: f32 = 2400.0;
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const SHIFTY_CIRCLE_STEP: f64 = 0.01;
pub const SHIFTY_CHANGE_STEP: f64 = 0.5;
pub const CLEAR_COLOR: Color = Color::rgb(0.149, 0.156, 0.290);
const SHIFTY_CIRCLE_RADIUS: f32 = 50.0;
const SHIFTY_CIRCLE_STROKE: f32 = 5.0;
const SHIFTY_CIRCLE_MIN_SPEED: f32 = 0.01;
const SHIFTY_CIRCLE_MAX_SPEED: f32 = 50.0;
// const SHIFTY_CIRCLE_FILL_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.2);
const SHIFTY_CIRCLE_FILL_COLOR: Color = Color::rgba(0.784, 0.713, 0.345, 0.01);
const SHIFTY_CIRCLE_STROKE_COLOR: Color = Color::rgba(0.784, 0.713, 0.345, 0.01);
#[cfg(target_arch = "wasm32")]
const RESIZE_CHECK_STEP: f64 = 1.0;


// Resource for app globals.
// Based on https://bevy-cheatbook.github.io/programming/res.html
#[derive(Default)]
pub struct AppGlobals {
    pub dest_low_x: f32,
    pub dest_high_x: f32,
    pub dest_low_y: f32,
    pub dest_high_y: f32,
}


#[derive(Component)]
pub struct ShiftyCircle;


// #[derive(Component, Clone, Copy, PartialEq, Eq)]
#[derive(Component)]
pub struct Destination {
    x: f32,
    y: f32,
    speed: f32,
}


pub fn setup_shifty_circle(mut commands: Commands) {
    // let mut rng = thread_rng();
    let mycircle = shapes::Circle {
        radius: SHIFTY_CIRCLE_RADIUS,
        ..Default::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &mycircle,
        DrawMode::Outlined {
            fill_mode: FillMode::color(SHIFTY_CIRCLE_FILL_COLOR),
            outline_mode: StrokeMode::new(SHIFTY_CIRCLE_STROKE_COLOR, SHIFTY_CIRCLE_STROKE),
        },
        Transform::default(),
    ))
    .insert(ShiftyCircle)
    .insert(Destination { x: 0.0, y: 0.0, speed: SHIFTY_CIRCLE_MIN_SPEED });
}


// Based on https://github.com/bevyengine/bevy/issues/175
// 
// Call the handle_browser_resize system once at startup (if window is created)
// to cover for the short period before handle_browser_resize kicks in
// (since that system will likely be set to a FixedTimeStep)
pub fn setup_browser_size(
    app_globals: ResMut<AppGlobals>,
    windows: ResMut<Windows>, 
    mut window_created_reader: EventReader<WindowCreated>
) {
    if window_created_reader.iter().next().is_some() {
        handle_browser_resize(windows, app_globals);
    }
}


// Based on this Discord conversation: https://i.imgur.com/osfA8PH.png AND
// https://github.com/mrk-its/bevy-robbo/blob/master/src/main.rs
pub fn handle_browser_resize(mut windows: ResMut<Windows>, mut app_globals: ResMut<AppGlobals>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
        app_globals.dest_low_x = -window.width() / 2.0 + SHIFTY_CIRCLE_RADIUS;
        app_globals.dest_high_x = window.width() / 2.0 - SHIFTY_CIRCLE_RADIUS;
        app_globals.dest_low_y = -window.height() / 2.0 + SHIFTY_CIRCLE_RADIUS;
        app_globals.dest_high_y = window.height() / 2.0 - SHIFTY_CIRCLE_RADIUS;
    }
}



pub fn translate_circle(mut q: Query<(&mut Transform, &Destination)>) {
    for (mut transform, dest) in q.iter_mut() {
        if dest.x > transform.translation.x {
            transform.translation.x += dest.speed;
        }
        if dest.x < transform.translation.x {
            transform.translation.x -= dest.speed;
        }

        if dest.y > transform.translation.y {
            transform.translation.y += dest.speed;
        }
        if dest.y < transform.translation.y {
            transform.translation.y -= dest.speed;
        }
    }

}


pub fn change_circle_destination(
    app_globals: Res<AppGlobals>, mut q: Query<&mut Destination, With<ShiftyCircle>>
) {
    let mut rng = thread_rng();
    for mut dest in q.iter_mut() {
        dest.x = rng.gen_range(app_globals.dest_low_x..app_globals.dest_high_x);
        dest.y = rng.gen_range(app_globals.dest_low_y..app_globals.dest_high_y);
        dest.speed = rng.gen_range(SHIFTY_CIRCLE_MIN_SPEED..SHIFTY_CIRCLE_MAX_SPEED);
        // println!("x: {}", dest.x);
        // println!("y: {}", dest.y);
        // println!("speed: {}", dest.speed);
        // println!("---");
    }
}


pub fn app() {
    // From https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
            title: "Shifty Circle".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#window-matching-canvas".to_string()),
            ..Default::default()
        }
    ).insert_resource(AppGlobals {
        dest_low_x: -WINDOW_WIDTH / 2.0,
        dest_high_x: WINDOW_WIDTH / 2.0,
        dest_low_y: -WINDOW_HEIGHT / 2.0,
        dest_high_y: WINDOW_HEIGHT / 2.0,
    }).insert_resource(ClearColor(CLEAR_COLOR))
    .insert_resource(Msaa { samples: 4 });

    info!("--Logging does not start before DefaultPlugins so this log won't appear--");
    app.add_plugins(DefaultPlugins);
    info!("--Logging has been set up in DefaultPlugins--");

    app.add_plugin(ShapePlugin);

    // Example of "feature-flipping". 
    // See https://doc.rust-lang.org/cargo/reference/features.html
    #[cfg(feature = "framestats")]
    app.add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.add_startup_system(setup_shifty_circle);

    #[cfg(target_arch = "wasm32")]
    app.add_startup_system(setup_browser_size)
    .add_system(
        handle_browser_resize.with_run_criteria(FixedTimestep::step(RESIZE_CHECK_STEP))
    );

    // Note setting with_run_criteria on a single system
    // (Found it here: https://bevy-cheatbook.github.io/programming/run-criteria.html#run-criteria-labels)
    app.add_system(
        translate_circle
            .with_run_criteria(FixedTimestep::step(SHIFTY_CIRCLE_STEP))
    ).add_system(
        change_circle_destination
            .with_run_criteria(FixedTimestep::step(SHIFTY_CHANGE_STEP))
    );

    app.run();
}
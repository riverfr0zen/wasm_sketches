use crate::base::sketch;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_web_extras::prelude::*;
use rand::prelude::thread_rng;
use rand::Rng;


// Place window on top right corner
const SHIFTY_CIRCLE_STEP: f64 = 0.01;
const SHIFTY_CHANGE_STEP: f64 = 1.5;
// const CLEAR_COLOR: Color = Color::rgb(0.149, 0.156, 0.290);
const CLEAR_COLOR: Color = Color::rgb(0.1, 0.11, 0.0);
const SHIFTY_CIRCLE_COUNT: u8 = 3;
const SHIFTY_CIRCLE_RADIUS: f32 = 40.0;
const SHIFTY_CIRCLE_STROKE: f32 = 1.0;
const SHIFTY_CIRCLE_MIN_SPEED: f32 = 0.01;
const SHIFTY_CIRCLE_MAX_SPEED: f32 = 25.0;
const SHIFTY_CIRCLE_FILL_COLOR: Color = Color::rgba(0.784, 0.713, 0.345, 0.0);
const SHIFTY_CIRCLE_STROKE_COLOR: Color = Color::rgba(0.784, 0.713, 0.345, 0.0);
const BUILDING_MIN_WIDTH: f32 = 10.0;
const BUILDING_MAX_WIDTH: f32 = 200.0;
// const BUILDING_COLOR: Color = Color::GREEN;
const BUILDING_FORE_COLOR: Color = Color::rgb(0.1, 0.09, 0.0);
const BUILDING_COLOR: Color = Color::rgb(0.1, 0.10, 0.0);
// Ratio of x below means the tallest building is 1/x screen height
const BUILDING_MAX_HEIGHT_RATIO: f32 = 2.0;
const BUILDING_MIN_HEIGHT_RATIO: f32 = 16.0;
const PULSATING_STEP: f64 = 0.1;
const PULSE_MAX_ALPHA: f32 = 0.1;
// const PULSE_SCALE: f64 = 0.1;
const PULSE_SCALE: f64 = 0.01;
const PULSE_AMPLITUDE: f64 = 1.0;
const PULSE_FREQ: f64 = 5.0;


// Resource for app globals.
// Based on https://bevy-cheatbook.github.io/programming/res.html
#[derive(Default, Debug)]
pub struct AppGlobals {
    dest_low_x: f32,
    dest_high_x: f32,
    dest_low_y: f32,
    dest_high_y: f32,
}


#[derive(Component)]
pub struct ShiftyCircle;


#[derive(Component)]
pub struct Building;


// #[derive(Component, Clone, Copy, PartialEq, Eq)]
#[derive(Component)]
pub struct Destination {
    x: f32,
    y: f32,
    speed: f32,
}


// Helpful on how to return multiple types:
// https://www.reddit.com/r/rust/comments/dme4og/can_we_return_multiple_type_data_from_the_function/
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=57223180ab43fff42e057d367468ac22
enum OneOf<A, B, C> {
    First(A),
    Second(B),
    Third(C),
}


enum ShiftyShapes {
    RECT,
    CIRCLE,
    ELLIPSE,
}


fn get_shape(
    shape: ShiftyShapes,
) -> OneOf<shapes::Circle, shapes::Ellipse, shapes::RegularPolygon> {
    match shape {
        ShiftyShapes::CIRCLE => {
            return OneOf::First(shapes::Circle {
                radius: SHIFTY_CIRCLE_RADIUS,
                ..Default::default()
            })
        }
        ShiftyShapes::ELLIPSE => {
            return OneOf::Second(shapes::Ellipse {
                radii: Vec2::new(SHIFTY_CIRCLE_RADIUS, SHIFTY_CIRCLE_RADIUS / 2.0),
                ..Default::default()
            })
        }
        ShiftyShapes::RECT => {
            return OneOf::Third(shapes::RegularPolygon {
                sides: 4,
                feature: shapes::RegularPolygonFeature::Radius(SHIFTY_CIRCLE_RADIUS),
                ..shapes::RegularPolygon::default()
            });
        }
    }
}


fn setup_shifty_circle(commands: Commands) {
    let some_shape = get_shape(ShiftyShapes::CIRCLE);
    /*
     * This way of destructuring took some time to figure out and is still is a little hard
     * to understand as I'm new to Rust.
     *
     * What it means is: "If let destructures `some_shape` into Either::Left(myshape), where
     * `myshape` would be the Circle shape we want, then run the `if` block. Need to do this
     * because I'm using the "Either" pattern for returning multiple types described here:
     *
     * https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=57223180ab43fff42e057d367468ac22
     * (Update: Changed "Either" to "OneOf")
     *
     * By using `if let` I can avoid the verbose and redundant match statement from the
     * previous commit of this function.
     *
     * Based on:
     * https://www.reddit.com/r/rust/comments/dme4og/can_we_return_multiple_type_data_from_the_function/
     */
    if let OneOf::First(myshape) = some_shape {
        setup_generic(commands, myshape);
    } else {
        panic!("Got the wrong shape!");
    }
}


fn setup_shifty_ufo(commands: Commands) {
    let some_shape = get_shape(ShiftyShapes::ELLIPSE);
    if let OneOf::Second(myshape) = some_shape {
        setup_generic(commands, myshape);
    } else {
        panic!("Got the wrong shape!");
    }
}

fn setup_shifty_rect(commands: Commands) {
    // For an example of triggering the panic below, you can ask for an ELLIPSE from `get_shape`
    // instead of the expected RECT
    // let some_shape = get_shape(ShiftyShapes::ELLIPSE);
    let some_shape = get_shape(ShiftyShapes::RECT);
    if let OneOf::Third(myshape) = some_shape {
        setup_generic(commands, myshape);
    } else {
        panic!("Got the wrong shape!");
    }
}


fn setup_generic(mut commands: Commands, myshape: impl Geometry) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // `_` means to discard the iterator element, since it's not being used:
    // https://stackoverflow.com/questions/29932503/what-is-the-idiomatic-way-to-write-a-for-loop-without-using-the-iterator-value
    //
    // `..=` is "inclusive ranges" notation:
    // https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#inclusive-ranges-with-
    for _ in 1..=SHIFTY_CIRCLE_COUNT {
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &myshape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(SHIFTY_CIRCLE_FILL_COLOR),
                    outline_mode: StrokeMode::new(SHIFTY_CIRCLE_STROKE_COLOR, SHIFTY_CIRCLE_STROKE),
                },
                // Transform::default(),
                Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ))
            .insert(ShiftyCircle)
            .insert(Destination {
                x: 0.0,
                y: 0.0,
                speed: SHIFTY_CIRCLE_MIN_SPEED,
            });
    }
}


fn draw_skyline_layer(
    commands: &mut Commands,
    available_space: f32,
    buildings_start_x: f32,
    buildings_start_y: f32,
    building_min_height: f32,
    building_max_height: f32,
    building_color: Color,
    z_index: f32,
) {
    let mut remaining_space = available_space;
    let mut building_pos_x = buildings_start_x;
    let mut rng = thread_rng();

    while remaining_space > 0.0 {
        // debug!("{:?}, {:?}", available_space, remaining_space);

        let building_width = if remaining_space > BUILDING_MAX_WIDTH {
            rng.gen_range(BUILDING_MIN_WIDTH..BUILDING_MAX_WIDTH)
        } else {
            BUILDING_MAX_WIDTH
        };
        let building_height = rng.gen_range(building_min_height..building_max_height);
        let building = shapes::Rectangle {
            extents: Vec2::new(building_width, building_height),
            origin: RectangleOrigin::BottomLeft,
            ..Default::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &building,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(building_color),
                    outline_mode: StrokeMode::new(SHIFTY_CIRCLE_STROKE_COLOR, SHIFTY_CIRCLE_STROKE),
                },
                // Transform::default(),
                Transform::from_translation(Vec3::new(building_pos_x, buildings_start_y, z_index)),
            ))
            .insert(Building);

        building_pos_x += building_width;
        remaining_space -= building_width;
    }
}

fn draw_skyline(
    mut commands: Commands,
    webcfg: ResMut<WebExtrasCfg>,
    mut q: Query<Entity, With<Building>>,
) {
    for entity in q.iter_mut() {
        commands.entity(entity).despawn();
    }

    let buildings_start_x = -webcfg.max_x;
    let buildings_start_y = -webcfg.max_y;
    let building_max_height = webcfg.height / BUILDING_MAX_HEIGHT_RATIO;
    let building_min_height = webcfg.height / BUILDING_MIN_HEIGHT_RATIO;

    draw_skyline_layer(
        &mut commands,
        webcfg.width,
        buildings_start_x,
        buildings_start_y,
        building_min_height,
        building_max_height,
        BUILDING_COLOR,
        0.0,
    );

    draw_skyline_layer(
        &mut commands,
        webcfg.width,
        buildings_start_x,
        buildings_start_y,
        building_min_height,
        building_max_height - building_max_height / 4.0,
        BUILDING_FORE_COLOR,
        2.0,
    );
}


fn translate_circle(mut q: Query<(&mut Transform, &Destination)>) {
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


fn change_circle_destination(
    app_globals: Res<AppGlobals>,
    mut q: Query<&mut Destination, With<ShiftyCircle>>,
) {
    let mut rng = thread_rng();
    for mut dest in q.iter_mut() {
        dest.x = rng.gen_range(app_globals.dest_low_x..app_globals.dest_high_x);
        dest.y = rng.gen_range(app_globals.dest_low_y..app_globals.dest_high_y);
        dest.speed = rng.gen_range(SHIFTY_CIRCLE_MIN_SPEED..SHIFTY_CIRCLE_MAX_SPEED);
    }
}


// Based on https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/dynamic_shape.rs
fn do_pulsating_effect(time: Res<Time>, mut query: Query<&mut DrawMode, With<ShiftyCircle>>) {
    // let hue = (time.seconds_since_startup() * 50.0) % 360.0;
    // let outline_width = 2.0 + time.seconds_since_startup().sin().abs() * 10.0;
    let secs_since = time.seconds_since_startup();
    let secs_theta = secs_since % 360.0;
    let pulse_wave = PULSE_AMPLITUDE * (secs_theta * PULSE_FREQ).sin().abs() * PULSE_SCALE;
    // debug!(
    //     "since: {:?}, theta: {:?}, pulse: {}",
    //     secs_since, secs_theta, pulse_wave
    // );

    for mut draw_mode in query.iter_mut() {
        // Helpful: https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
        if let DrawMode::Outlined {
            ref mut fill_mode,
            ref mut outline_mode,
        } = *draw_mode
        {
            if pulse_wave > PULSE_MAX_ALPHA as f64 {
                fill_mode.color.set_a(PULSE_MAX_ALPHA);
                outline_mode.color.set_a(PULSE_MAX_ALPHA);
            } else {
                fill_mode.color.set_a(pulse_wave as f32);
                outline_mode.color.set_a(pulse_wave as f32);
            }
        }
    }
}


#[cfg(target_arch = "wasm32")]
fn handle_post_browser_resize(
    commands: Commands,
    webcfg: ResMut<WebExtrasCfg>,
    mut resize_event_reader: EventReader<BrowserResized>,
    mut app_globals: ResMut<AppGlobals>,
    buildings_query: Query<Entity, With<Building>>,
) {
    if resize_event_reader.iter().next().is_some() {
        app_globals.dest_low_x = -webcfg.max_x + SHIFTY_CIRCLE_RADIUS;
        app_globals.dest_high_x = webcfg.max_x - SHIFTY_CIRCLE_RADIUS;
        app_globals.dest_low_y = -webcfg.max_y + SHIFTY_CIRCLE_RADIUS;
        app_globals.dest_high_y = webcfg.max_y - SHIFTY_CIRCLE_RADIUS;
        draw_skyline(commands, webcfg, buildings_query);
    }
}


pub fn app(variation: &str) {
    let webcfg = WebExtrasCfg {
        title: format!("shifty{}", String::from(variation)),
        match_clear_color_always: true,
        ..Default::default()
    };
    // Need to copy a couple of values here b/c webcfg will be lost to `web_app`
    let webcfg_max_x = webcfg.max_x;
    let webcfg_max_y = webcfg.max_y;
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(AppGlobals {
            dest_low_x: -webcfg_max_x,
            dest_high_x: webcfg_max_x,
            dest_low_y: -webcfg_max_y,
            dest_high_y: webcfg_max_y,
        })
        .add_plugin(ShapePlugin);


    match variation {
        "ufo" => app.add_startup_system(setup_shifty_ufo),
        "rect" => app.add_startup_system(setup_shifty_rect),
        _ => app.add_startup_system(setup_shifty_circle),
    };

    // If wasm32, the skyline will be drawn in handle_post_browser_resize
    #[cfg(not(target_arch = "wasm32"))]
    app.add_startup_system(draw_skyline);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_post_browser_resize);


    // Note setting with_run_criteria on a single system
    // (Found it here: https://bevy-cheatbook.github.io/programming/run-criteria.html#run-criteria-labels)
    app.add_system(translate_circle.with_run_criteria(FixedTimestep::step(SHIFTY_CIRCLE_STEP)))
        .add_system(
            change_circle_destination.with_run_criteria(FixedTimestep::step(SHIFTY_CHANGE_STEP)),
        )
        .add_system(do_pulsating_effect.with_run_criteria(FixedTimestep::step(PULSATING_STEP)));

    app.run();
}

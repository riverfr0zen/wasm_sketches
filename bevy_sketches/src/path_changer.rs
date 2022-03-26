use crate::base::prelude::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::thread_rng;
use rand::Rng;


/*
 * path_changer
 *
 * Instructed by:
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/dynamic_shape.rs
 */


pub const CHANGER_STEP: f64 = 1.0;
pub const CHANGER_CLEAR_CLR: Color = Color::BLUE;
const CHANGER_FILL_CLR: Color = Color::MIDNIGHT_BLUE;
const CHANGER_STROKE_CLR: Color = Color::BLACK;
const CHANGER_STROKE: f32 = 5.0;
const CHANGER_MAX_SEGMENTS: u8 = 120;

pub fn path_changing_eg_setup(mut commands: Commands) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);
    path_builder.line_to(100.0 * Vec2::ONE);

    path_builder.line_to(Vec2::new(100.0, 0.0));
    path_builder.close();

    /*
     * Irf: Temporary workaround until the fix mentioned in this issue is released:
     * https://github.com/Nilirad/bevy_prototype_lyon/issues/138
     */
    // let line = path_builder.build();
    let line = path_builder.build().0;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &line,
        // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
        DrawMode::Outlined {
            fill_mode: FillMode::color(CHANGER_FILL_CLR),
            outline_mode: StrokeMode::new(CHANGER_STROKE_CLR, CHANGER_STROKE),
        },
        Transform::default(),
    ));
}

/// Function to safeguard against invalid arguments when generating random range
fn gen_random_safely(start: f32, end: f32) -> f32 {
    let mut rng = thread_rng();

    if start >= end {
        info!("!!! s {}, e {}", start, end);
        return end;
    }
    return rng.gen_range(start..end);

    // Tried returning as i16 to somewhat improve results (not as many small floating point
    // variationsthat don't translate to screen). But dunno if it does much good.
    // return rng.gen_range(start as i16..end as i16) as f32;
}

/// @TODO Good enough for now, but can be improved.
///
/// Currently the randomization reaches the end of range in a quadrant too quickly,
/// so even if you increase number of segments, this function reaches the end of the range sooner.
///
/// One idea for improvement is to break down each quadrant to sub-ranges for gen_range().
pub fn path_changer(winsetup: Res<WindowSetup>, mut query: Query<&mut Path>) {
    let mut rng = thread_rng();

    let num_segments = rng.gen_range(3..=CHANGER_MAX_SEGMENTS);
    let mut path_builder = PathBuilder::new();

    let mut last_x: f32 = 0.0;
    let mut last_y: f32 = 0.0;
    let mut current_quad = 1;

    // @HINT
    // Using an underscore to discard the iterator value since it's not being used
    for _i in 1..=num_segments {
        let segment_place: f32 = _i as f32 / num_segments as f32;

        if segment_place <= 0.25 {
            if last_x == 0.0 {
                info!("entered quad 1");
                last_x = gen_random_safely(-winsetup.max_x, 0.0);
                last_y = gen_random_safely(0.0, winsetup.max_y);
                path_builder.move_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(last_x, 0.0);
                // For quads 1 & 3 using `last_y` seems less necessary in terms of
                // edges crossing each other. Crossing does occur, but much less
                // frequently. However, using it does eliminate all crossing, seemingly
                // at the cost of more "conservative shapes" (but I have not verified it).
                last_y = gen_random_safely(last_y, winsetup.max_y);
                // last_y = rng.gen_range(0.0..winsetup.max_y);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            info!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            continue;
        }

        if segment_place > 0.25 && segment_place <= 0.5 {
            if current_quad < 2 {
                info!("entered quad 2");
                last_x = gen_random_safely(0.0, winsetup.max_x);
                last_y = gen_random_safely(0.0, winsetup.max_y);
                path_builder.line_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(last_x, winsetup.max_x);
                // In quads 2 & 4, where the horizontal direction of the shape generation
                // changes on the next quadrant, it seems that using `last_y` in the range
                // makes better shapes (edges don't cross)
                //
                // last_y = rng.gen_range(0.0..winsetup.max_y);
                last_y = gen_random_safely(0.0, last_y);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            info!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            current_quad = 2;
            continue;
        }

        if segment_place > 0.5 && segment_place <= 0.75 {
            if current_quad < 3 {
                info!("entered quad 3");
                last_x = gen_random_safely(0.0, winsetup.max_x);
                last_y = gen_random_safely(-winsetup.max_y, 0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(0.0, last_x);
                // For quads 1 & 3 using `last_y` seems less necessary in terms of
                // edges crossing each other. Crossing does occur, but much less
                // frequently. However, using it does eliminate all crossing, seemingly
                // at the cost of more "conservative shapes" (but I have not verified it).
                last_y = gen_random_safely(-winsetup.max_y, last_y);
                // last_y = rng.gen_range(-winsetup.max_y..0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            info!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            current_quad = 3;
            continue;
        }

        if segment_place > 0.75 && segment_place <= 1.0 {
            // Check if it's the first time in this quadrant
            if current_quad < 4 {
                info!("entered quad 4");
                last_x = gen_random_safely(-winsetup.max_x, 0.0);
                last_y = gen_random_safely(-winsetup.max_y, 0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            } else {
                last_x = gen_random_safely(-winsetup.max_x, last_x);
                // In quads 2 & 4, where the horizontal direction of the shape generation
                // changes on the next quadrant, it seems that using `last_y` in the range
                // makes better shapes (edges don't cross)
                //
                // last_y = rng.gen_range(-winsetup.max_y..0.0);
                last_y = gen_random_safely(last_y, 0.0);
                path_builder.line_to(Vec2::new(last_x, last_y));
            }
            info!(
                "---i: {}, segment_place: {}, x: {}, y: {}",
                _i, segment_place, last_x, last_y
            );
            current_quad = 4;
            continue;
        }
    }
    info!("--end-{} segments shape---\n", num_segments);
    path_builder.close();
    let new_path = path_builder.build().0;

    let mut path = query.iter_mut().next().unwrap();
    *path = ShapePath::build_as(&new_path);
}


pub fn app() {
    let winsetup = WindowSetup {
        clear_color: CHANGER_CLEAR_CLR,
        ..Default::default()
    };
    let mut app = sketch_factory(winsetup);

    app.add_plugin(ShapePlugin)
        .add_startup_system(path_changing_eg_setup)
        .add_system(path_changer.with_run_criteria(FixedTimestep::step(CHANGER_STEP)))
        .run();
}

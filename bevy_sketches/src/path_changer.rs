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


pub const CHANGER_STEP: f64 = 0.5;
pub const CHANGER_CLEAR_CLR: Color = Color::MIDNIGHT_BLUE;
const CHANGER_FILL_CLR: Color = Color::ORANGE;
const CHANGER_STROKE_CLR: Color = Color::BLACK;
const CHANGER_STROKE: f32 = 10.0;
const CHANGER_MAX_SEGMENTS: u8 = 8;

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


pub fn path_changer(winsetup: Res<WindowSetup>, mut query: Query<&mut Path>) {
    let mut rng = thread_rng();

    let num_segments = rng.gen_range(2..CHANGER_MAX_SEGMENTS);
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);

    // @HINT
    // Using an underscore to discard the iterator value since it's not being used
    for _i in 0..num_segments {
        // if i / num_vertices <= .25:
        // (next vertex should be: x: range(-maxX, 0), y: range(0, maxY))

        // # top right
        // if i / num_vertices > .25 and i / num_vertices <= .5:
        //     (next vertex should be: x: range(0, maxX), y: range(0, maxY))

        // # bottom right
        // if i / num_vertices > .5 and i / num_vertices <= .75:
        //     (next vertex should be: x: range(0, maxX), y: range(-maxY, 0))

        // # bottom left
        // if i / num_vertices > .75 and i / num_vertices <= 1.0:
        //     (next vertex should be: x: range(-maxX, 0), y: range(-maxY, 0))

        let segment_place: f32 = _i as f32 / num_segments as f32;

        if segment_place <= 0.25 {
            path_builder.line_to(Vec2::new(
                rng.gen_range(-winsetup.max_x..0.0),
                rng.gen_range(0.0..winsetup.max_y),
            ));
            continue;
        }

        if segment_place > 0.25 && segment_place <= 0.5 {
            path_builder.line_to(Vec2::new(
                rng.gen_range(0.0..winsetup.max_x),
                rng.gen_range(0.0..winsetup.max_y),
            ));
        }

        if segment_place > 0.5 && segment_place <= 0.75 {
            path_builder.line_to(Vec2::new(
                rng.gen_range(0.0..winsetup.max_x),
                rng.gen_range(-winsetup.max_y..0.0),
            ));
        }

        if segment_place > 0.75 && segment_place <= 1.0 {
            path_builder.line_to(Vec2::new(
                rng.gen_range(-winsetup.max_x..0.0),
                rng.gen_range(-winsetup.max_y..0.0),
            ));
        }

        // path_builder.line_to(Vec2::new(
        //     rng.gen_range(-winsetup.max_x..winsetup.max_x),
        //     rng.gen_range(-winsetup.max_y..winsetup.max_y),
        // ));
    }
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

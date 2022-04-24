use crate::base::sketch;
use crate::shapegen::random_polygon_builder;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_web_extras::prelude::*;

/*
 * path_changer
 *
 * Instructed by:
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/dynamic_shape.rs
 */


pub const CHANGER_STEP: f64 = 0.5;
pub const CHANGER_CLEAR_CLR: Color = Color::BLUE;
const CHANGER_FILL_CLR: Color = Color::MIDNIGHT_BLUE;
const CHANGER_STROKE_CLR: Color = Color::BLACK;
const CHANGER_STROKE: f32 = 5.0;
const CHANGER_MAX_SEGMENTS: u8 = 32;


pub fn path_changing_eg_setup(webcfg: Res<WebExtrasCfg>, mut commands: Commands) {
    let path_builder = random_polygon_builder(webcfg.max_x, webcfg.max_y, CHANGER_MAX_SEGMENTS);
    let shape = path_builder.build().0;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
        DrawMode::Outlined {
            fill_mode: FillMode::color(CHANGER_FILL_CLR),
            outline_mode: StrokeMode::new(CHANGER_STROKE_CLR, CHANGER_STROKE),
        },
        Transform::default(),
    ));
}


pub fn path_changer(webcfg: Res<WebExtrasCfg>, mut query: Query<&mut Path>) {
    let path_builder = random_polygon_builder(webcfg.max_x, webcfg.max_y, CHANGER_MAX_SEGMENTS);

    //  * Irf: Temporary workaround until the fix mentioned in this issue is released:
    //  * https://github.com/Nilirad/bevy_prototype_lyon/issues/138
    let new_shape = path_builder.build().0;

    let mut path = query.iter_mut().next().unwrap();
    *path = ShapePath::build_as(&new_shape);
}


pub fn app() {
    let webcfg = WebExtrasCfg {
        title: String::from("path changer example"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(CHANGER_CLEAR_CLR))
        .add_plugin(ShapePlugin)
        .add_startup_system(path_changing_eg_setup)
        .add_system(path_changer.with_run_criteria(FixedTimestep::step(CHANGER_STEP)))
        .run();
}

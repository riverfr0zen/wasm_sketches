use crate::base::sketch;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_web_extras::prelude::*;
use rand::prelude::thread_rng;
use rand::Rng;


/*
 * curve_eg
 *
 * Instructed by:
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/src/path.rs
 */

pub const CURVE_CLEAR_CLR: Color = Color::DARK_GRAY;
const CURVE_FILL_CLR: Color = Color::ORANGE;
const CURVE_STROKE_CLR: Color = Color::BLACK;
const CURVE_STROKE: f32 = 5.0;
const CELL_CTRL_X: f32 = 200.0;
const CELL_CTRL_Y: f32 = 200.0;
const CELL_RADIUS: f32 = 50.0; // Radius to curve intersection
const CELL_MAX_RADIUS: f32 = 300.0;
pub const CELL_STEP: f64 = 1.0;


#[derive(Component)]
pub struct Cell {
    ctrl_x: f32,
    ctrl_y: f32,
    radius: f32,
    radius_target: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ctrl_x: CELL_CTRL_X,
            ctrl_y: CELL_CTRL_Y,
            radius: CELL_RADIUS,
            radius_target: CELL_RADIUS,
        }
    }
}


fn gen_cell_path(cell: &Cell) -> PathBuilder {
    let mut path_builder = PathBuilder::new();

    // Right side top
    path_builder.move_to(Vec2::new(0.0, cell.radius));
    path_builder.quadratic_bezier_to(
        Vec2::new(cell.ctrl_x, cell.ctrl_y),
        Vec2::new(cell.radius, 0.0),
    );

    // Right side bottom
    path_builder.quadratic_bezier_to(
        Vec2::new(cell.ctrl_x, -cell.ctrl_y),
        Vec2::new(0.0, -cell.radius),
    );

    // Left side bottom
    path_builder.quadratic_bezier_to(
        Vec2::new(-cell.ctrl_x, -cell.ctrl_y),
        Vec2::new(-cell.radius, 0.0),
    );

    // Left side top
    path_builder.quadratic_bezier_to(
        Vec2::new(-cell.ctrl_x, cell.ctrl_y),
        Vec2::new(0.0, cell.radius),
    );
    path_builder.close();
    return path_builder;
}


fn cell_setup(mut commands: Commands) {
    let cell = Cell::default();
    let path_builder = gen_cell_path(&cell);

    let path = path_builder.build().0;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &path,
            // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
            DrawMode::Outlined {
                fill_mode: FillMode::color(CURVE_FILL_CLR),
                outline_mode: StrokeMode::new(CURVE_STROKE_CLR, CURVE_STROKE),
            },
            Transform::default(),
        ))
        .insert(cell);
}


fn redraw_cell(mut query: Query<(&mut Path, &mut Cell)>) {
    let (mut path, mut cell) = query.iter_mut().next().unwrap();
    if cell.radius < cell.radius_target {
        cell.radius += 1.0;
    } else if cell.radius > cell.radius_target {
        cell.radius -= 1.0;
    }

    let path_builder = gen_cell_path(&cell);
    //  * Irf: Temporary workaround until the fix mentioned in this issue is released:
    //  * https://github.com/Nilirad/bevy_prototype_lyon/issues/138
    let new_path = path_builder.build().0;

    *path = ShapePath::build_as(&new_path);
}


fn mutate_cell(mut query: Query<&mut Cell>) {
    let mut rng = thread_rng();
    let mut cell = query.iter_mut().next().unwrap();
    cell.radius_target = rng.gen_range(CELL_RADIUS..CELL_MAX_RADIUS);
}

pub fn app() {
    let webcfg = WebExtrasCfg::default();
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(Color::rgb(0.72, 0.81, 1.0)))
        .add_plugin(ShapePlugin)
        .add_startup_system(cell_setup)
        .add_system(redraw_cell)
        .add_system(mutate_cell.with_run_criteria(FixedTimestep::step(CELL_STEP)));

    app.run();
}

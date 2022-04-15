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

pub const CELL_CLEAR_CLR: Color = Color::rgb(0.58, 0.71, 0.87);
const CELL_FILL_CLR: Color = Color::rgba(0.95, 0.85, 0.62, 0.2);
const CELL_STROKE_CLR: Color = Color::rgba(0.95, 0.91, 0.81, 0.2);
const CELL_STROKE: f32 = 5.0;
const CELL_CTRL_MIN: f32 = 200.0;
const CELL_CTRL_MAX: f32 = 800.0;
/// Radius to curve intersection
const CELL_MIN_RADIUS: f32 = 50.0;
/// It seems that keeping radius size between 100-125% of **the smaller** of ctrl.x or
/// ctrl.y keeps the shape from getting too sharp, at least on the concave "surfaces".
const CELL_MAX_RADIUS_MODIFIER: f32 = 1.10;
const CELL_SEG_RT: usize = 0;
const CELL_SEG_RB: usize = 1;
const CELL_SEG_LB: usize = 2;
const CELL_SEG_LT: usize = 3;
/// Although these speed values are used for both radius and ctrl speeds, the ctrl max speed is
/// nerfed in the `mutate_cell` system
const CELL_MIN_SPEED: f32 = 1.0;
const CELL_MAX_SPEED: f32 = 20.0;
pub const CELL_STEP: f64 = 0.1;
// pub const CELL_STEP: f64 = 1.0;


#[derive(Component)]
pub struct CellSegment {
    ctrl: Vec2,
    ctrl_target: Vec2,
    ctrl_speed: f32,
    radius: f32,
    radius_target: f32,
    radius_speed: f32,
}

impl CellSegment {
    fn get_max_radius(&self) -> f32 {
        if self.ctrl.x > self.ctrl.y {
            return self.ctrl.y * CELL_MAX_RADIUS_MODIFIER;
        }
        return self.ctrl.x * CELL_MAX_RADIUS_MODIFIER;
    }
}

impl Default for CellSegment {
    fn default() -> Self {
        Self {
            ctrl: Vec2::new(CELL_CTRL_MIN, CELL_CTRL_MIN),
            ctrl_speed: CELL_MIN_SPEED,
            ctrl_target: Vec2::new(CELL_CTRL_MIN, CELL_CTRL_MIN),
            radius: CELL_MIN_RADIUS,
            radius_target: CELL_MIN_RADIUS,
            radius_speed: CELL_MIN_SPEED,
        }
    }
}

#[derive(Component)]
pub struct Cell {
    segments: [CellSegment; 4],
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            segments: [
                CellSegment::default(),
                CellSegment::default(),
                CellSegment::default(),
                CellSegment::default(),
            ],
        }
    }
}

fn gen_cell_path(cell: &Cell) -> PathBuilder {
    let mut path_builder = PathBuilder::new();

    // Right side top
    path_builder.move_to(Vec2::new(0.0, cell.segments[CELL_SEG_LT].radius));
    path_builder.quadratic_bezier_to(
        Vec2::new(
            cell.segments[CELL_SEG_RT].ctrl.x,
            cell.segments[CELL_SEG_RT].ctrl.y,
        ),
        Vec2::new(cell.segments[CELL_SEG_RT].radius, 0.0),
    );

    // Right side bottom
    path_builder.quadratic_bezier_to(
        Vec2::new(
            cell.segments[CELL_SEG_RB].ctrl.x,
            -cell.segments[CELL_SEG_RB].ctrl.y,
        ),
        Vec2::new(0.0, -cell.segments[CELL_SEG_RB].radius),
    );

    // Left side bottom
    path_builder.quadratic_bezier_to(
        Vec2::new(
            -cell.segments[CELL_SEG_LB].ctrl.x,
            -cell.segments[CELL_SEG_LB].ctrl.y,
        ),
        Vec2::new(-cell.segments[CELL_SEG_LB].radius, 0.0),
    );

    // Left side top
    path_builder.quadratic_bezier_to(
        Vec2::new(
            -cell.segments[CELL_SEG_LT].ctrl.x,
            cell.segments[CELL_SEG_LT].ctrl.y,
        ),
        // Need to close up cleanly so we are going back to values from the RT segment
        // Vec2::new(0.0, cell.segments[3].radius),
        Vec2::new(0.0, cell.segments[CELL_SEG_LT].radius),
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
                fill_mode: FillMode::color(CELL_FILL_CLR),
                outline_mode: StrokeMode::new(CELL_STROKE_CLR, CELL_STROKE),
            },
            Transform::default(),
        ))
        .insert(cell);
}


fn get_next_location(current_location: f32, target_location: f32, speed: f32) -> f32 {
    if current_location == target_location {
        return current_location;
    }

    if current_location < target_location {
        let next_location = current_location + speed;
        if next_location > target_location {
            return target_location;
        }
        return next_location;
    } else {
        let next_location = current_location - speed;
        if next_location < target_location {
            return target_location;
        }
        return next_location;
    }
}


fn redraw_cell(mut query: Query<(&mut Path, &mut Cell)>) {
    let (mut path, mut cell) = query.iter_mut().next().unwrap();
    for seg in &mut cell.segments {
        seg.ctrl.x = get_next_location(seg.ctrl.x, seg.ctrl_target.x, seg.ctrl_speed);
        seg.ctrl.y = get_next_location(seg.ctrl.y, seg.ctrl_target.y, seg.ctrl_speed);
        seg.radius = get_next_location(seg.radius, seg.radius_target, seg.radius_speed);
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

    for seg in &mut cell.segments {
        seg.radius_speed = rng.gen_range(CELL_MIN_SPEED..CELL_MAX_SPEED);
        seg.ctrl_speed = rng.gen_range(CELL_MIN_SPEED..CELL_MAX_SPEED / 2.0);
        seg.ctrl_target.x = rng.gen_range(CELL_CTRL_MIN..CELL_CTRL_MAX);
        seg.ctrl_target.y = rng.gen_range(CELL_CTRL_MIN..CELL_CTRL_MAX);
        seg.radius_target = rng.gen_range(CELL_MIN_RADIUS..seg.get_max_radius());
    }
}

pub fn app() {
    let webcfg = WebExtrasCfg::default();
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(CELL_CLEAR_CLR))
        .add_plugin(ShapePlugin)
        .add_startup_system(cell_setup)
        .add_system(redraw_cell)
        .add_system(mutate_cell.with_run_criteria(FixedTimestep::step(CELL_STEP)));

    app.run();
}

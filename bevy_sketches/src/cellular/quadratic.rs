use crate::base::sketch;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_web_extras::prelude::*;
use rand::prelude::thread_rng;
use rand::Rng;

/*
 * cellular
 *
 * Instructed by:
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/src/path.rs
 */

pub const CELL_CLEAR_CLR: Color = Color::rgb(0.58, 0.71, 0.87);
const CELL_FILL_CLR: Color = Color::rgba(0.95, 0.85, 0.62, 0.1);
const CELL_STROKE_CLR: Color = Color::rgba(0.95, 0.91, 0.81, 0.08);
const CELL_STROKE: f32 = 5.0;
const CELL_INNER_FILL_CLR: Color = Color::rgba(1.0, 0.79, 0.69, 0.2);
const CELL_INNER_STROKE_CLR: Color = Color::rgba(0.41, 0.1, 0.03, 0.1);
const CELL_INNER_STROKE: f32 = 2.0;
const CELL_INNER_SIZE: f32 = 0.9;
const CELL_CTRL_MIN: f32 = 100.0;
const CELL_CTRL_MAX: f32 = 250.0;
/// Radius to curve intersection
/// Setting the CELL_MIN_RADIUS closer to CELL_CTRL_MIN lessens the valleys in the shape
const CELL_MIN_RADIUS: f32 = 100.0;
/// It seems that keeping radius size between 100-125% of **the smaller** of ctrl.x or
/// ctrl.y keeps the shape from getting too sharp, at least on the concave "surfaces".
const CELL_MAX_RADIUS_MODIFIER: f32 = 1.10;
/// Less tight causes more convex / peaks (used only if max_radius_looser)
const CELL_MAX_RADIUS_TIGHTNESS: f32 = 1.5;
const CELL_SEG_RT: usize = 0;
const CELL_SEG_RB: usize = 1;
const CELL_SEG_LB: usize = 2;
const CELL_SEG_LT: usize = 3;
/// Although these speed values are used for both radius and ctrl speeds, the ctrl max speed is
/// nerfed in the `mutate_cell` system
const CELL_MIN_SPEED: f32 = 1.0;
const CELL_MAX_SPEED: f32 = 4.0;
pub const CELL_STEP: f64 = 0.3;
// pub const CELL_STEP: f64 = 1.0;


#[derive(Component)]
pub struct CellSegment {
    ctrl: Vec2,
    ctrl_target: Vec2,
    ctrl_speed: f32,
    radius: f32,
    radius_target: f32,
    radius_speed: f32,
    max_radius_looser: bool,
}


impl CellSegment {
    fn get_max_radius(&self) -> f32 {
        if self.max_radius_looser {
            return self.get_looser_max_radius();
        }

        let max_radius;
        if self.ctrl.x > self.ctrl.y {
            max_radius = self.ctrl.y * CELL_MAX_RADIUS_MODIFIER;
        } else {
            max_radius = self.ctrl.x * CELL_MAX_RADIUS_MODIFIER;
        }
        if max_radius <= CELL_MIN_RADIUS {
            return CELL_MIN_RADIUS + 1.0;
        }
        return max_radius;
    }

    /// Get a max radius that is not as gated by the smaller of ctrl.x and ctrl.y
    fn get_looser_max_radius(&self) -> f32 {
        let max_radius = (self.ctrl.x + self.ctrl.y) / CELL_MAX_RADIUS_TIGHTNESS;
        if max_radius <= CELL_MIN_RADIUS {
            return CELL_MIN_RADIUS + 1.0;
        }
        return max_radius;
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
            max_radius_looser: true,
        }
    }
}


#[derive(Component)]
pub struct Cell {
    segments: [CellSegment; 4],
}


impl Cell {
    fn tight() -> Self {
        Self {
            segments: [
                CellSegment {
                    max_radius_looser: false,
                    ..Default::default()
                },
                CellSegment {
                    max_radius_looser: false,
                    ..Default::default()
                },
                CellSegment {
                    max_radius_looser: false,
                    ..Default::default()
                },
                CellSegment {
                    max_radius_looser: false,
                    ..Default::default()
                },
            ],
        }
    }
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


#[derive(Component)]
pub struct CellInner;


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


fn spawn_cell(commands: &mut Commands, cell: Cell, translation: Vec3) -> Entity {
    let path_builder = gen_cell_path(&cell);
    let path = path_builder.build();

    let cell_bundle = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &path,
            // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
            DrawMode::Outlined {
                fill_mode: FillMode::color(CELL_FILL_CLR),
                outline_mode: StrokeMode::new(CELL_STROKE_CLR, CELL_STROKE),
            },
            Transform {
                translation: translation,
                ..Default::default()
            },
        ))
        .insert(cell)
        .id();

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &path,
            // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
            DrawMode::Outlined {
                fill_mode: FillMode::color(CELL_INNER_FILL_CLR),
                outline_mode: StrokeMode::new(CELL_INNER_STROKE_CLR, CELL_INNER_STROKE),
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, translation.z - 1.0),
                scale: Vec3::new(CELL_INNER_SIZE, CELL_INNER_SIZE, 1.0),
                ..Default::default()
            },
        ))
        .insert(CellInner)
        .insert(Parent(cell_bundle));

    return cell_bundle;
}


fn cell_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    spawn_cell(&mut commands, Cell::default(), Vec3::ONE);
    spawn_cell(
        &mut commands,
        Cell::default(),
        Vec3::new(-350.0, 320.0, 3.0),
    );
    spawn_cell(&mut commands, Cell::tight(), Vec3::new(-200.0, -250.0, 2.0));
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


// With help from
// https://bevy-cheatbook.github.io/features/parent-child.html
// https://github.com/bevyengine/bevy/blob/main/examples/ecs/hierarchy.rs
fn redraw_cell(
    mut query: Query<(&mut Path, &mut Cell, &Children)>,
    mut inner_cell_query: Query<(&mut Path, With<CellInner>), Without<Cell>>,
) {
    for (mut path, mut cell, children) in query.iter_mut() {
        for seg in &mut cell.segments {
            seg.ctrl.x = get_next_location(seg.ctrl.x, seg.ctrl_target.x, seg.ctrl_speed);
            seg.ctrl.y = get_next_location(seg.ctrl.y, seg.ctrl_target.y, seg.ctrl_speed);
            seg.radius = get_next_location(seg.radius, seg.radius_target, seg.radius_speed);
        }
        let path_builder = gen_cell_path(&cell);
        let new_path = path_builder.build();
        *path = ShapePath::build_as(&new_path);

        let &inner_cell_entity = children.iter().next().unwrap();
        if let Ok((mut inner_path, _)) = inner_cell_query.get_mut(inner_cell_entity) {
            *inner_path = ShapePath::build_as(&new_path);
        }
    }
}


fn mutate_cell(mut query: Query<&mut Cell>) {
    let mut rng = thread_rng();

    for mut cell in query.iter_mut() {
        for seg in &mut cell.segments {
            seg.radius_speed = rng.gen_range(CELL_MIN_SPEED..CELL_MAX_SPEED);
            seg.ctrl_speed = rng.gen_range(CELL_MIN_SPEED..CELL_MAX_SPEED / 2.0);
            seg.ctrl_target.x = rng.gen_range(CELL_CTRL_MIN..CELL_CTRL_MAX);
            seg.ctrl_target.y = rng.gen_range(CELL_CTRL_MIN..CELL_CTRL_MAX);
            seg.radius_target = rng.gen_range(CELL_MIN_RADIUS..seg.get_max_radius());
        }
    }
}


pub fn app() {
    let webcfg = WebExtrasCfg {
        title: String::from("cellular"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(CELL_CLEAR_CLR))
        .add_plugin(ShapePlugin)
        .add_startup_system(cell_setup)
        .add_system(redraw_cell)
        .add_system(mutate_cell.with_run_criteria(FixedTimestep::step(CELL_STEP)));

    app.run();
}

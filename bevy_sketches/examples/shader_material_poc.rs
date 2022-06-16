use bevy::{
    prelude::*,
    render::{RenderApp, RenderStage},
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_sketches::base::sketch;
use bevy_sketches::shader_material::{update_time, ShaderMaterial};
use bevy_web_extras::prelude::*;


pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
const SURFACE_WIDTH: f32 = 500.0;
const SURFACE_HEIGHT: f32 = 800.0;


pub fn main() {
    let webcfg = WebExtrasCfg {
        title: String::from("cellular"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    // app.insert_resource(ClearColor(CELL_CLEAR_CLR))
    //     .add_plugin(ShapePlugin)
    //     .add_startup_system(cell_setup)
    //     .add_system(redraw_cell)
    //     .add_system(mutate_cell.with_run_criteria(FixedTimestep::step(CELL_STEP)));
    app.insert_resource(ClearColor(Color::SALMON))
        .add_plugin(Material2dPlugin::<ShaderMaterial>::default())
        .add_startup_system(poc_setup);

    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, update_time);


    app.run();
}


fn poc_setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<ShaderMaterial>>,
    webcfg: Res<WebExtrasCfg>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            scale: Vec3::new(SURFACE_WIDTH, SURFACE_HEIGHT, 1.0),
            translation: Vec3::new(0.0, -webcfg.max_y + SURFACE_HEIGHT / 2.0, 1.0),
            ..Transform::default()
        },
        material: my_material_assets.add(ShaderMaterial::default()),
        ..default()
    });

    const surface2_height: f32 = SURFACE_HEIGHT * 1.5;
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            scale: Vec3::new(SURFACE_WIDTH, surface2_height, 1.0),
            translation: Vec3::new(501.0, -webcfg.max_y + surface2_height / 2.0, 1.0),
            ..Default::default()
        },
        material: my_material_assets.add(ShaderMaterial::default()),
        ..default()
    });
}

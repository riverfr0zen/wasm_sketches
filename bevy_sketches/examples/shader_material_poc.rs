use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_sketches::base::sketch;
use bevy_sketches::shader_materials::{
    core::{DisplayQuad, ShaderMaterialPlugin},
    eg_material::ExampleMaterial,
    eg_res_material::ResExampleMaterial,
};
use bevy_web_extras::prelude::*;


pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
const SURFACE_WIDTH: f32 = 500.0;
const SURFACE_HEIGHT: f32 = 800.0;
const SURFACE2_WIDTH: f32 = SURFACE_WIDTH;
const SURFACE2_HEIGHT: f32 = SURFACE_HEIGHT * 1.5;
const SURFACE3_WIDTH: f32 = SURFACE_WIDTH * 1.5;
const SURFACE3_HEIGHT: f32 = SURFACE_HEIGHT / 2.0;


pub fn main() {
    let webcfg = WebExtrasCfg {
        title: String::from("cellular"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(Color::SALMON))
        .add_plugin(ShaderMaterialPlugin::<ExampleMaterial>::default())
        .add_plugin(ShaderMaterialPlugin::<ResExampleMaterial>::default())
        .add_startup_system(poc_setup);

    app.run();
}


fn poc_setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut eg_material_assets: ResMut<Assets<ExampleMaterial>>,
    mut res_eg_material_assets: ResMut<Assets<ResExampleMaterial>>,
    webcfg: Res<WebExtrasCfg>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


    // @TODO positioning is messed up on Web because it should be drawn on resize
    // (see conditional setup of systems in shifty circle)
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(SURFACE_WIDTH, SURFACE_HEIGHT, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x + (SURFACE_WIDTH / 2.0) + 10.0,
                    -webcfg.max_y + (SURFACE_HEIGHT / 2.0),
                    1.0,
                ),
                ..Transform::default()
            },
            material: eg_material_assets.add(ExampleMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(SURFACE_WIDTH, SURFACE2_HEIGHT, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x + (SURFACE2_WIDTH / 2.0) + SURFACE_WIDTH + 20.0,
                    -webcfg.max_y + SURFACE2_HEIGHT / 2.0,
                    1.0,
                ),
                ..Default::default()
            },
            material: res_eg_material_assets.add(ResExampleMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(SURFACE3_WIDTH, SURFACE3_HEIGHT, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x + (SURFACE3_WIDTH / 2.0) + SURFACE_WIDTH + SURFACE2_WIDTH + 30.0,
                    -webcfg.max_y + SURFACE3_HEIGHT / 2.0,
                    1.0,
                ),
                ..Default::default()
            },
            material: res_eg_material_assets.add(ResExampleMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);
}

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_sketches::base::sketch;
use bevy_sketches::shader_materials::{
    core::{DisplayQuad, ShaderMaterialPlugin},
    eg_material::ExampleMaterial,
    eg_mo_data_material::AdditionalDataMaterial,
    eg_res_material::ResExampleMaterial,
};
use bevy_web_extras::prelude::*;


pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
const SURFACE_WIDTH: f32 = 500.0;
const SURFACE_HEIGHT: f32 = 800.0;
const SURFACE2_WIDTH: f32 = SURFACE_WIDTH;
// const SURFACE2_HEIGHT: f32 = SURFACE_HEIGHT * 1.5;
const SURFACE2_HEIGHT: f32 = SURFACE_HEIGHT;
const SURFACE3_WIDTH: f32 = SURFACE_WIDTH * 1.25;
const SURFACE3_HEIGHT: f32 = SURFACE_HEIGHT / 2.0;
const SURFACE4_WIDTH: f32 = SURFACE_WIDTH;
const SURFACE4_HEIGHT: f32 = SURFACE_HEIGHT / 2.0;


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
        .add_plugin(ShaderMaterialPlugin::<AdditionalDataMaterial>::default());

    // If wasm32, the skyline will be drawn in handle_post_browser_resize
    #[cfg(not(target_arch = "wasm32"))]
    app.add_startup_system(poc_setup);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_post_browser_resize);

    app.run();
}


#[cfg(target_arch = "wasm32")]
fn handle_post_browser_resize(
    commands: Commands,
    mut entity_q: Query<Entity, With<DisplayQuad>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut eg_material_assets: ResMut<Assets<ExampleMaterial>>,
    mut res_eg_material_assets: ResMut<Assets<ResExampleMaterial>>,
    mut eg_mo_data_material_assets: ResMut<Assets<AdditionalDataMaterial>>,
    mut resize_event_reader: EventReader<BrowserResized>,
    webcfg: Res<WebExtrasCfg>,
) {
    if resize_event_reader.iter().next().is_some() {
        poc_setup(
            commands,
            entity_q,
            mesh_assets,
            eg_material_assets,
            res_eg_material_assets,
            eg_mo_data_material_assets,
            webcfg,
        )
    }
}


fn poc_setup(
    mut commands: Commands,
    mut entity_q: Query<Entity, With<DisplayQuad>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut eg_material_assets: ResMut<Assets<ExampleMaterial>>,
    mut res_eg_material_assets: ResMut<Assets<ResExampleMaterial>>,
    mut eg_mo_data_material_assets: ResMut<Assets<AdditionalDataMaterial>>,
    webcfg: Res<WebExtrasCfg>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // In case we're redrawing due to a resize, let's clear out the previous entities
    for entity in entity_q.iter_mut() {
        commands.entity(entity).despawn();
    }

    // @TODO positioning is messed up on Web because it should be drawn on resize
    // (see conditional setup of systems in shifty circle)
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(SURFACE_WIDTH, SURFACE_HEIGHT, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x + (SURFACE_WIDTH / 2.0) + 10.0,
                    webcfg.max_y - (SURFACE_HEIGHT / 2.0),
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
                    webcfg.max_y - (SURFACE2_HEIGHT / 2.0),
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
                    webcfg.max_y - (SURFACE3_HEIGHT / 2.0),
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
                scale: Vec3::new(SURFACE4_WIDTH, SURFACE4_HEIGHT, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x
                        + (SURFACE4_WIDTH / 2.0)
                        + SURFACE_WIDTH
                        + SURFACE2_WIDTH
                        + SURFACE3_WIDTH
                        + 40.0,
                    webcfg.max_y - (SURFACE4_HEIGHT / 2.0),
                    1.0,
                ),
                ..Default::default()
            },
            material: eg_mo_data_material_assets.add(AdditionalDataMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);


    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(SURFACE4_WIDTH, SURFACE4_HEIGHT - 10.0, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x
                        + (SURFACE4_WIDTH / 2.0)
                        + SURFACE_WIDTH
                        + SURFACE2_WIDTH
                        + SURFACE3_WIDTH
                        + 40.0,
                    webcfg.max_y - (SURFACE4_HEIGHT / 2.0) - SURFACE4_HEIGHT,
                    1.0,
                ),
                ..Default::default()
            },
            material: eg_mo_data_material_assets.add(AdditionalDataMaterial::with_rects(3)),
            ..default()
        })
        .insert(DisplayQuad);
}

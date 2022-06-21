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
// const SURFACE_PADDING: f32 = 0.009;
// const SURFACE_WIDTH: f32 = 0.18;
const SURFACE_PADDING: f32 = 0.001;
const SURFACE_WIDTH: f32 = 0.1895;
const SURFACE_HEIGHT: f32 = 0.995;
const SURFACE2_WIDTH: f32 = SURFACE_WIDTH;
// const SURFACE2_HEIGHT: f32 = SURFACE_HEIGHT * 1.5;
const SURFACE2_HEIGHT: f32 = SURFACE_HEIGHT;
const SURFACE3_WIDTH: f32 = SURFACE_WIDTH * 1.25;
const SURFACE3_HEIGHT: f32 = SURFACE_HEIGHT * 0.9;
const SURFACE4_WIDTH: f32 = SURFACE_WIDTH;
const SURFACE4_HEIGHT: f32 = SURFACE_HEIGHT;


pub fn main() {
    let webcfg = WebExtrasCfg {
        title: String::from("reusable shader material proof of concept"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    // app.insert_resource(ClearColor(Color::rgb(0.08, 0.24, 0.33)))
    app.insert_resource(ClearColor(Color::rgb(0.3, 0.41, 0.48)))
        .add_plugin(ShaderMaterialPlugin::<ExampleMaterial>::default())
        .add_plugin(ShaderMaterialPlugin::<ResExampleMaterial>::default())
        .add_plugin(ShaderMaterialPlugin::<AdditionalDataMaterial>::default());

    // If wasm32, this will be handled in handle_post_browser_resize
    #[cfg(not(target_arch = "wasm32"))]
    app.add_startup_system(poc_setup);

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_post_browser_resize);

    app.run();
}


#[cfg(target_arch = "wasm32")]
fn handle_post_browser_resize(
    commands: Commands,
    entity_q: Query<Entity, With<DisplayQuad>>,
    mesh_assets: ResMut<Assets<Mesh>>,
    eg_material_assets: ResMut<Assets<ExampleMaterial>>,
    res_eg_material_assets: ResMut<Assets<ResExampleMaterial>>,
    eg_mo_data_material_assets: ResMut<Assets<AdditionalDataMaterial>>,
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


fn to_scale(webcfg: &Res<WebExtrasCfg>, normalized: Vec2) -> Vec2 {
    Vec2::new(normalized.x * webcfg.width, normalized.y * webcfg.height)
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

    // Don't need padding height, so just passing 1.0
    let padding_wh = to_scale(&webcfg, Vec2::new(SURFACE_PADDING, 1.0));

    let surface1_wh = to_scale(&webcfg, Vec2::new(SURFACE_WIDTH, SURFACE_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface1_wh.x, surface1_wh.y, 1.0),
                // translation: Vec3::new(-webcfg.max_x + (surface1_wh.x / 2.0) + 10.0, 0.0, 0.0),
                translation: Vec3::new(
                    -webcfg.max_x + (surface1_wh.x / 2.0) + padding_wh.x,
                    0.0,
                    0.0,
                ),
                ..Transform::default()
            },
            material: eg_material_assets.add(ExampleMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);

    let surface2_wh = to_scale(&webcfg, Vec2::new(SURFACE2_WIDTH, SURFACE2_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface2_wh.x, surface2_wh.y, 1.0),
                translation: Vec3::new(
                    // -webcfg.max_x + (surface2_wh.x / 2.0) + surface1_wh.x + 20.0,
                    -webcfg.max_x + (surface2_wh.x / 2.0) + surface1_wh.x + (padding_wh.x * 2.0),
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            material: res_eg_material_assets.add(ResExampleMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);

    let surface3_wh = to_scale(&webcfg, Vec2::new(SURFACE3_WIDTH, SURFACE3_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface3_wh.x, surface3_wh.y, 1.0),
                translation: Vec3::new(
                    // -webcfg.max_x + (surface3_wh.x / 2.0) + surface1_wh.x + surface2_wh.x + 30.0,
                    -webcfg.max_x
                        + (surface3_wh.x / 2.0)
                        + surface1_wh.x
                        + surface2_wh.x
                        + (padding_wh.x * 3.0),
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            material: res_eg_material_assets.add(ResExampleMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);

    let surface4_wh = to_scale(&webcfg, Vec2::new(SURFACE4_WIDTH, SURFACE4_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface4_wh.x, surface4_wh.y, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x
                        + (surface4_wh.x / 2.0)
                        + surface1_wh.x
                        + surface2_wh.x
                        + surface3_wh.x
                        // + 40.0,
                        + (padding_wh.x * 4.0),
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            material: eg_mo_data_material_assets.add(AdditionalDataMaterial::default()),
            ..default()
        })
        .insert(DisplayQuad);

    let surface5_wh = to_scale(&webcfg, Vec2::new(SURFACE4_WIDTH, SURFACE4_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface5_wh.x, surface5_wh.y, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x
                        + (surface5_wh.x / 2.0)
                        + surface1_wh.x
                        + surface2_wh.x
                        + surface3_wh.x
                        + surface4_wh.x
                        // + 40.0,
                        + (padding_wh.x * 5.0),
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            material: eg_mo_data_material_assets.add(AdditionalDataMaterial::with_rects(3)),
            ..default()
        })
        .insert(DisplayQuad);
}

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_sketches::base::sketch;
use bevy_sketches::shader_materials::{
    building_lights::{BuildingLights, BuildingLightsUniform},
    core::{color_to_shader_vec3, DisplayQuad, ShaderMaterialPlugin},
};
use bevy_web_extras::prelude::*;


const SURFACE_WIDTH: f32 = 0.25;
const SURFACE_HEIGHT: f32 = 1.0;


pub fn main() {
    let webcfg = WebExtrasCfg {
        title: String::from("scaling net shader"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugin(ShaderMaterialPlugin::<BuildingLights>::default());

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
    material_assets: ResMut<Assets<ScalingNet>>,
    mut resize_event_reader: EventReader<BrowserResized>,
    webcfg: Res<WebExtrasCfg>,
) {
    if resize_event_reader.iter().next().is_some() {
        poc_setup(commands, entity_q, mesh_assets, material_assets, webcfg)
    }
}


fn to_scale(webcfg: &Res<WebExtrasCfg>, normalized: Vec2) -> Vec2 {
    Vec2::new(normalized.x * webcfg.width, normalized.y * webcfg.height)
}


fn poc_setup(
    mut commands: Commands,
    mut entity_q: Query<Entity, With<DisplayQuad>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<BuildingLights>>,
    webcfg: Res<WebExtrasCfg>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // In case we're redrawing due to a resize, let's clear out the previous entities
    for entity in entity_q.iter_mut() {
        commands.entity(entity).despawn();
    }

    let surface1_wh = to_scale(&webcfg, Vec2::new(SURFACE_WIDTH, SURFACE_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface1_wh.x, surface1_wh.y, 1.0),
                translation: Vec3::new(-webcfg.max_x + surface1_wh.x / 2.0, 0.0, 0.0),
                ..Default::default()
            },
            material: material_assets.add(BuildingLights::default()),
            ..default()
        })
        .insert(DisplayQuad);

    let surface2_wh = to_scale(&webcfg, Vec2::new(SURFACE_WIDTH, SURFACE_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface2_wh.x, surface2_wh.y, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x + surface2_wh.x / 2.0 + surface1_wh.x,
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            material: material_assets.add(BuildingLights {
                uniform: BuildingLightsUniform {
                    background_color: color_to_shader_vec3(Color::MIDNIGHT_BLUE),
                    alpha: 0.95,
                    rand_modifier: 100.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        })
        .insert(DisplayQuad);


    let surface3_wh = to_scale(&webcfg, Vec2::new(SURFACE_WIDTH * 2.0, SURFACE_HEIGHT));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                scale: Vec3::new(surface3_wh.x, surface3_wh.y, 1.0),
                translation: Vec3::new(
                    -webcfg.max_x + surface3_wh.x / 2.0 + surface1_wh.x + surface2_wh.x,
                    0.0,
                    0.0,
                ),
                ..Default::default()
            },
            material: material_assets.add(BuildingLights {
                uniform: BuildingLightsUniform {
                    background_color: color_to_shader_vec3(Color::MIDNIGHT_BLUE),
                    rand_modifier: 500.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        })
        .insert(DisplayQuad);
}

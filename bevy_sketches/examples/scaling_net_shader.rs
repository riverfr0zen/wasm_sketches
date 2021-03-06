use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_sketches::base::sketch;
use bevy_sketches::shader_materials::{
    core::{DisplayQuad, ShaderMaterialPlugin},
    scaling_net::ScalingNet,
};
use bevy_web_extras::prelude::*;


const SURFACE_WIDTH: f32 = 1.0;
const SURFACE_HEIGHT: f32 = 1.0;


pub fn main() {
    let webcfg = WebExtrasCfg {
        title: String::from("scaling net shader"),
        match_element: Some(String::from("content")),
        match_clear_color: true,
        ..Default::default()
    };
    let mut app = sketch(webcfg);
    app.insert_resource(ClearColor(Color::SEA_GREEN))
        .add_plugin(ShaderMaterialPlugin::<ScalingNet>::default());

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
    scaling_net_assets: ResMut<Assets<ScalingNet>>,
    mut resize_event_reader: EventReader<BrowserResized>,
    webcfg: Res<WebExtrasCfg>,
) {
    if resize_event_reader.iter().next().is_some() {
        poc_setup(commands, entity_q, mesh_assets, scaling_net_assets, webcfg)
    }
}


fn to_scale(webcfg: &Res<WebExtrasCfg>, normalized: Vec2) -> Vec2 {
    Vec2::new(normalized.x * webcfg.width, normalized.y * webcfg.height)
}


fn poc_setup(
    mut commands: Commands,
    mut entity_q: Query<Entity, With<DisplayQuad>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut scaling_net_assets: ResMut<Assets<ScalingNet>>,
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
                ..Transform::default()
            },
            material: scaling_net_assets.add(ScalingNet::default()),
            ..default()
        })
        .insert(DisplayQuad);
}

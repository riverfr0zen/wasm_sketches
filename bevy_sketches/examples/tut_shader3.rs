// Exploring uniform data: time & resolution in a Uniform struct, as shown here:
// https://github.com/mwbryant/logic-projects-bevy-shader-tutorial/blob/changing-values/src/main.rs#:~:text=let%20uniform_data%20%3D,buffer.as_entire_binding()%2C
//
// Based on the following tutorials/discussions:
//
// https://github.com/mwbryant/logic-projects-bevy-shader-tutorial/blob/basic-shaders/src/main.rs
//
// https://www.youtube.com/watch?v=_hX37bsdYao
//
// ... but mainly settling on:
//
// https://discord.com/channels/691052431525675048/742884593551802431/970382134663577661


#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            // BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
            std140::{AsStd140, Std140},
            BindGroup,
            BindGroupDescriptor,
            BindGroupEntry,
            BindGroupLayout,
            BindGroupLayoutDescriptor,
            BindGroupLayoutEntry,
            BindingType,
            BufferBindingType,
            BufferInitDescriptor,
            BufferSize,
            BufferUsages,
            ShaderStages,
        },
        renderer::RenderDevice,
        RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
    window::PresentMode,
};
use std::mem::size_of;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
const SURFACE_WIDTH: f32 = 800.0;
const SURFACE_HEIGHT: f32 = 800.0;
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader2_material.wgsl";
const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_shaping_line.wgsl";


fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<MyMaterial>::default())
        .add_startup_system(spawn_quad)
        .add_startup_system(spawn_camera);

    app.sub_app_mut(RenderApp)
        .add_system_to_stage(RenderStage::Extract, update_time);

    app.run();
}

fn spawn_quad(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<MyMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            scale: Vec3::new(SURFACE_WIDTH, SURFACE_HEIGHT, 1.0),
            ..Transform::default()
        },
        material: my_material_assets.add(MyMaterial::default()),
        ..default()
    });
}

#[derive(TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct MyMaterial {
    pub time: f32,
}

impl Default for MyMaterial {
    fn default() -> Self {
        Self { time: 0.0 }
    }
}

struct MyMaterialGPU {
    bind_group: BindGroup,
}

impl Material2d for MyMaterial {
    fn bind_group(material: &MyMaterialGPU) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                // @TODO: vertex_shader: Re-enable VERTEX_FRAGMENT when exploring
                // visibility: ShaderStages::VERTEX_FRAGMENT,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(size_of::<f32>() as u64),
                },
                count: None,
            }],
        })
    }

    // @TODO: vertex_shader: The Discord discussion introduces (to me) a vertex_shader.
    // I want to first get time working, then I'll revisit this. Be sure to see the other
    // vertex_shader todo above as well.
    //
    // fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    //     asset_server.watch_for_changes().unwrap();
    //     Some(asset_server.load(MATERIAL_PATH))
    // }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load(MATERIAL_PATH))
    }
}


impl RenderAsset for MyMaterial {
    type ExtractedAsset = MyMaterial;
    type PreparedAsset = MyMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<MyMaterial>>);

    fn extract_asset(&self) -> MyMaterial {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: MyMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<MyMaterialGPU, PrepareAssetError<MyMaterial>> {
        let time_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: extracted_asset.time.as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
        });
        Ok(MyMaterialGPU { bind_group })
    }
}


fn update_time(mut mat_query: ResMut<Assets<MyMaterial>>, time: Res<Time>) {
    for (_, mut mymaterial) in mat_query.iter_mut() {
        mymaterial.time = time.seconds_since_startup() as f32;
    }
}


fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    // camera.orthographic_projection.right = 1.0 * RESOLUTION;
    // camera.orthographic_projection.left = -1.0 * RESOLUTION;

    // camera.orthographic_projection.top = 1.0;
    // camera.orthographic_projection.bottom = -1.0;

    // camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}

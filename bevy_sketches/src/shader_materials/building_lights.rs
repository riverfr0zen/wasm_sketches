/// Demonstrates providing additional uniform data from material to shader. See also the
/// accompanying shader at `MATERIAL_PATH` below.
///
/// Steps to creating a new material
/// 1. First, follow all the steps listed in `super::eg_material.rs`
/// 2..?
use super::core::{BaseShaderTrait, CommonUniformData};
use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
            BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline},
};

// const MATERIAL_PATH: &str = "poc_shaders/rects_from_additional_data.wgsl";
const MATERIAL_PATH: &str = "shiftyc/building_lights.wgsl";
const DEFAULT_BGCOLOR: (f32, f32, f32) = (0.5, 0.5, 0.5);


#[derive(Clone, AsStd140)]
pub struct SomeCustomUniformData {
    pub common: CommonUniformData,
    pub background_color: Vec3,
}


#[derive(TypeUuid, Clone)]
#[uuid = "f305c425-4f41-40cf-b7d3-b6e4a1ed6f04"]
pub struct BuildingLights {
    uniform: SomeCustomUniformData,
}

impl BuildingLights {
    pub fn with_config(background_color: Vec3) -> Self {
        Self {
            uniform: SomeCustomUniformData {
                common: CommonUniformData::default(),
                background_color: background_color,
            },
        }
    }
}

impl Default for BuildingLights {
    fn default() -> Self {
        Self {
            uniform: SomeCustomUniformData {
                common: CommonUniformData::default(),
                background_color: Vec3::new(
                    DEFAULT_BGCOLOR.0,
                    DEFAULT_BGCOLOR.1,
                    DEFAULT_BGCOLOR.2,
                ),
            },
        }
    }
}

impl BaseShaderTrait for BuildingLights {
    fn set_time(&mut self, time: f32) {
        self.uniform.common.time = time;
    }

    fn set_resolution(&mut self, resolution: Vec2) {
        self.uniform.common.resolution = resolution;
    }
}


pub struct GPUBuildingLights {
    bind_group: BindGroup,
}


impl Material2d for BuildingLights {
    fn bind_group(material: &GPUBuildingLights) -> &BindGroup {
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
                    min_binding_size: BufferSize::new(
                        SomeCustomUniformData::std140_size_static() as u64
                    ),
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


impl RenderAsset for BuildingLights {
    type ExtractedAsset = BuildingLights;
    type PreparedAsset = GPUBuildingLights;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<BuildingLights>>);

    fn extract_asset(&self) -> BuildingLights {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: BuildingLights,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<GPUBuildingLights, PrepareAssetError<BuildingLights>> {
        // let uniform_data = CommonUniformData {
        //     time: extracted_asset.uniform.common.time,
        //     resolution: extracted_asset.uniform.common.resolution,
        // };
        let uniform_data = SomeCustomUniformData {
            common: CommonUniformData {
                time: extracted_asset.uniform.common.time,
                resolution: extracted_asset.uniform.common.resolution,
            },
            background_color: extracted_asset.uniform.background_color,
        };

        let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: uniform_data.as_std140().as_bytes(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        Ok(GPUBuildingLights { bind_group })
    }
}

/// Demonstrates providing additional uniform data from material to shader. See also the
/// accompanying shader at `MATERIAL_PATH` below.
///
/// Steps to creating a new material
/// 1. Copy the code below to a new source file
/// 2. Globally replace "AdditionalDataMaterial" with the name of the new material struct
/// 3. Generate a new `uuid` and replace the one used for the ExampleMaterial struct
/// 4. Change the MATERIAL_PATH constant below to the shader you want.
/// 5. Update SomeCustomUniformData to contain the data you want. Update other structs,
///    impls, etc. also to reflect these changes.
/// 6. Make sure you also update the shader to resemble the new custom uniform data.
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

const MATERIAL_PATH: &str = "poc_shaders/rects_from_additional_data.wgsl";
const DEFAULT_NUM_RECTS: u32 = 2;


#[derive(Clone, AsStd140)]
pub struct SomeCustomUniformData {
    pub common: CommonUniformData,
    pub num_rects: u32,
}


#[derive(TypeUuid, Clone)]
#[uuid = "9ae754a8-7c86-45e7-87d6-601a11a703f0"]
pub struct AdditionalDataMaterial {
    uniform: SomeCustomUniformData,
}

impl AdditionalDataMaterial {
    pub fn with_rects(num_rects: u32) -> Self {
        Self {
            uniform: SomeCustomUniformData {
                common: CommonUniformData::default(),
                num_rects: num_rects,
            },
        }
    }
}

impl Default for AdditionalDataMaterial {
    fn default() -> Self {
        Self {
            uniform: SomeCustomUniformData {
                common: CommonUniformData::default(),
                num_rects: DEFAULT_NUM_RECTS,
            },
        }
    }
}

impl BaseShaderTrait for AdditionalDataMaterial {
    fn set_time(&mut self, time: f32) {
        self.uniform.common.time = time;
    }

    fn set_resolution(&mut self, resolution: Vec2) {
        self.uniform.common.resolution = resolution;
    }
}


pub struct GPUAdditionalDataMaterial {
    bind_group: BindGroup,
}


impl Material2d for AdditionalDataMaterial {
    fn bind_group(material: &GPUAdditionalDataMaterial) -> &BindGroup {
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


impl RenderAsset for AdditionalDataMaterial {
    type ExtractedAsset = AdditionalDataMaterial;
    type PreparedAsset = GPUAdditionalDataMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<AdditionalDataMaterial>>,
    );

    fn extract_asset(&self) -> AdditionalDataMaterial {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: AdditionalDataMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<GPUAdditionalDataMaterial, PrepareAssetError<AdditionalDataMaterial>> {
        let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: extracted_asset.uniform.as_std140().as_bytes(),
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
        Ok(GPUAdditionalDataMaterial { bind_group })
    }
}

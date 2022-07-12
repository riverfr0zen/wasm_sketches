use super::core::{BaseShaderMaterial, BaseShaderTrait, CommonUniformData};
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

const MATERIAL_PATH: &str = "poc_shaders/erratic_wave.wgsl";


#[derive(TypeUuid, Clone)]
#[uuid = "ba185d29-c899-4acb-b140-94c787c779a0"]
pub struct ErraticWaveMaterial(BaseShaderMaterial);


impl Default for ErraticWaveMaterial {
    fn default() -> Self {
        Self(BaseShaderMaterial::default())
    }
}


impl BaseShaderTrait for ErraticWaveMaterial {
    fn set_time(&mut self, time: f32) {
        self.0.uniform.time = time;
    }

    fn set_resolution(&mut self, resolution: Vec2) {
        self.0.uniform.resolution = resolution;
    }
}


pub struct GPUErraticWaveMaterial {
    bind_group: BindGroup,
}


impl Material2d for ErraticWaveMaterial {
    fn bind_group(material: &GPUErraticWaveMaterial) -> &BindGroup {
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
                        CommonUniformData::std140_size_static() as u64
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


impl RenderAsset for ErraticWaveMaterial {
    type ExtractedAsset = ErraticWaveMaterial;
    type PreparedAsset = GPUErraticWaveMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<ErraticWaveMaterial>>,
    );

    fn extract_asset(&self) -> ErraticWaveMaterial {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: ErraticWaveMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<GPUErraticWaveMaterial, PrepareAssetError<ErraticWaveMaterial>> {
        let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: extracted_asset.0.uniform.as_std140().as_bytes(),
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
        Ok(GPUErraticWaveMaterial { bind_group })
    }
}

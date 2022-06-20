/// Material boilterplate for shader that demonstrates using resolution data. See shader
/// located at MATERIAL_PATH below for more details.
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

const MATERIAL_PATH: &str = "poc_shaders/res_rects.wgsl";


#[derive(TypeUuid, Clone)]
#[uuid = "cf1490ee-136c-4603-a22e-3e4ed0ae6acb"]
pub struct ResExampleMaterial(BaseShaderMaterial);


impl Default for ResExampleMaterial {
    fn default() -> Self {
        Self(BaseShaderMaterial::default())
    }
}


impl BaseShaderTrait for ResExampleMaterial {
    fn set_time(&mut self, time: f32) {
        self.0.uniform.time = time;
    }

    fn set_resolution(&mut self, resolution: Vec2) {
        self.0.uniform.resolution = resolution;
    }
}


pub struct GPUResExampleMaterial {
    bind_group: BindGroup,
}


impl Material2d for ResExampleMaterial {
    fn bind_group(material: &GPUResExampleMaterial) -> &BindGroup {
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


impl RenderAsset for ResExampleMaterial {
    type ExtractedAsset = ResExampleMaterial;
    type PreparedAsset = GPUResExampleMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<ResExampleMaterial>>,
    );

    fn extract_asset(&self) -> ResExampleMaterial {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: ResExampleMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<GPUResExampleMaterial, PrepareAssetError<ResExampleMaterial>> {
        let uniform_data = CommonUniformData {
            time: extracted_asset.0.uniform.time,
            resolution: extracted_asset.0.uniform.resolution,
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
        Ok(GPUResExampleMaterial { bind_group })
    }
}

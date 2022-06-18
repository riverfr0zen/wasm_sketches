use super::core::{BaseShaderMaterial, BaseShaderTrait};
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
    },
    sprite::{Material2d, Material2dPipeline},
};
use std::mem::size_of;


// const MATERIAL_PATH: &str = "tut_shaders/tut_shader2_material.wgsl";
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_shaping_line.wgsl";
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_shaping_trig.wgsl";
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_colors.wgsl";
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_colors_mix.wgsl";
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_shapes_rect.wgsl";
// const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_shapes_rect2.wgsl";
const MATERIAL_PATH: &str = "tut_shaders/tut_shader3_building_lights.wgsl";


#[derive(TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct ExampleMaterial(BaseShaderMaterial);


impl Default for ExampleMaterial {
    fn default() -> Self {
        Self(BaseShaderMaterial::default())
    }
}


impl BaseShaderTrait for ExampleMaterial {
    fn set_time(&mut self, time: f32) {
        // @TODO: Would be nice to have a cleaner way of accessing time
        self.0.time = time;
    }
}


pub struct GPUExampleMaterial {
    bind_group: BindGroup,
}


impl Material2d for ExampleMaterial {
    fn bind_group(material: &GPUExampleMaterial) -> &BindGroup {
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


impl RenderAsset for ExampleMaterial {
    type ExtractedAsset = ExampleMaterial;
    type PreparedAsset = GPUExampleMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<ExampleMaterial>>,
    );

    fn extract_asset(&self) -> ExampleMaterial {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: ExampleMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<GPUExampleMaterial, PrepareAssetError<ExampleMaterial>> {
        let time_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            // @TODO: Would be nice to have a cleaner way of accessing time
            // contents: extracted_asset.time.as_bytes(),
            contents: extracted_asset.0.time.as_bytes(),
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
        Ok(GPUExampleMaterial { bind_group })
    }
}
/*
 * Shader material DRY is not completely there yet. So far I have a plugin that can use
 * multiple materials, and a "BaseShaderMaterial". There's still a lot of boiler plate code in each
 * material.
 *
 * I've made some attempts make things more generic and reusable, mainly through traits, but I'm
 * still figuring out how to achieve or replace "inheritance" in Rust. Any solutions also have to
 * jive with the surrounding Bevy materials architecture, which have also posed some puzzles for me
 * due to lack my familiarity.
 *
 *
 * Some resources I've been looking at around traits and "inheritance":
 *
 * https://stackoverflow.com/questions/52418809/can-a-trait-give-a-default-implementation-for-the-method-of-a-trait-that-it-inhe
 *
 * https://stackoverflow.com/questions/70186164/can-a-trait-give-default-implementation-for-some-methods-of-a-parent-trait
 *
 * https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel#:~:text=Another%20alternative%20is%20to%20use%20generics%3A
 */


use bevy::{
    prelude::*,
    render::{render_resource::std140::AsStd140, RenderApp, RenderStage},
    sprite::{Material2d, Material2dPlugin},
};
use std::marker::PhantomData;


#[derive(Clone, AsStd140)]
pub struct CommonUniformData {
    // @TODO Add resolution, mouse, etc.
    pub time: f32,
    pub resolution: Vec2,
}

impl Default for CommonUniformData {
    fn default() -> Self {
        Self {
            time: 0.0,
            resolution: Vec2::ONE,
        }
    }
}


pub trait BaseShaderTrait: Material2d {
    fn set_time(&mut self, time: f32);

    fn set_resolution(&mut self, resolution: Vec2);
}


#[derive(Clone)]
pub struct BaseShaderMaterial {
    // pub time: f32,
    pub uniform: CommonUniformData,
}


impl Default for BaseShaderMaterial {
    fn default() -> Self {
        Self {
            uniform: CommonUniformData::default(),
        }
    }
}


pub struct ShaderMaterialPlugin<T: BaseShaderTrait>(PhantomData<T>);


impl<M: BaseShaderTrait> Default for ShaderMaterialPlugin<M> {
    fn default() -> Self {
        Self(Default::default())
    }
}


impl<T: BaseShaderTrait> Plugin for ShaderMaterialPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<T>::default())
            .sub_app_mut(RenderApp)
            // .add_system_to_stage(RenderStage::Extract, update_time::<T>)
            .add_system_to_stage(RenderStage::Extract, update_common_uniform_data::<T>);
    }
}


// Original update system which just updated time. Left here for reference.
// pub fn update_time<T: BaseShaderTrait>(mut mat_resources: ResMut<Assets<T>>, time: Res<Time>) {
//     for (_, mymaterial) in mat_resources.iter_mut() {
//         mymaterial.set_time(time.seconds_since_startup() as f32);
//     }
// }


#[derive(Component)]
pub struct DisplayQuad;


/// Update uniform data (time & resolution) in material for sending to shader
pub fn update_common_uniform_data<T: BaseShaderTrait>(
    time: Res<Time>,
    mut mat_resources: ResMut<Assets<T>>,
    // Figured out by looking at the declaration of MaterialMesh2dBundle that you can
    // query for the handle of the material. With the handleId from this, we can filter
    // Assets<T> to get and update the particular material asset.
    quad_query: Query<(&Transform, &Handle<T>), With<DisplayQuad>>,
) {
    for (asset_handle, mymaterial) in mat_resources.iter_mut() {
        mymaterial.set_time(time.seconds_since_startup() as f32);

        for (transform, handle) in quad_query.iter() {
            if handle.id == asset_handle {
                mymaterial.set_resolution(Vec2::new(transform.scale.x, transform.scale.y));
            }
        }
    }
}

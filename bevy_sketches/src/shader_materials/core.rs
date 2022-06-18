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
 * Some resources that have been helpful:
 *
 * https://stackoverflow.com/questions/52418809/can-a-trait-give-a-default-implementation-for-the-method-of-a-trait-that-it-inhe
 *
 * https://stackoverflow.com/questions/70186164/can-a-trait-give-default-implementation-for-some-methods-of-a-parent-trait
 *
 * https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel#:~:text=Another%20alternative%20is%20to%20use%20generics%3A
 */


use bevy::{
    prelude::*,
    render::{RenderApp, RenderStage},
    sprite::{Material2d, Material2dPlugin},
};
use std::marker::PhantomData;


pub trait BaseShaderTrait: Material2d {
    fn set_time(&mut self, time: f32);
}


#[derive(Clone)]
pub struct BaseShaderMaterial {
    pub time: f32,
}


impl Default for BaseShaderMaterial {
    fn default() -> Self {
        Self { time: 0.0 }
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
            .add_system_to_stage(RenderStage::Extract, update_time::<T>);
    }
}

pub fn update_time<T: BaseShaderTrait>(mut mat_query: ResMut<Assets<T>>, time: Res<Time>) {
    for (_, mymaterial) in mat_query.iter_mut() {
        mymaterial.set_time(time.seconds_since_startup() as f32);
    }
}

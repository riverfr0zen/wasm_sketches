// From: https://github.com/mwbryant/logic-projects-bevy-shader-tutorial/blob/basic-shaders/assets/my_material.wgsl
//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    let xpos: f32 = input.uv.x * 800.0;
    if (
        xpos < 50.0 || 
        (xpos > 100.0 && xpos < 200.0) || 
        (xpos > 400.0 && xpos < 600.0) ||
        (xpos > 700.0 && xpos < 800.0)
    ) {
        var output_color = vec4<f32>(input.uv.x, input.uv.y,0.0,1.0);
        return output_color;
    } else {
        var output_color = vec4<f32>(input.uv.y, input.uv.x,0.0,1.0);
        return output_color;
    }

    // var output_color = vec4<f32>(input.uv,0.0,1.0);
    // return output_color;
}

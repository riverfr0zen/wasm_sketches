// Demonstrates a shader that receives and uses data in addition to the CommonUniformData

#import "shader_common/common_uniform.wgsl"
#import "shader_common/shapefuncs.wgsl"


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};


// Nesting CommonUnifromData in a CustomUniformData struct
struct CustomUniformData {
    common: CommonUniformData;
    background_color: vec3<f32>;
};


[[group(1), binding(0)]]
var<uniform> u: CustomUniformData;


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    // var backgroundColor: vec3<f32> = vec3<f32>(0.01, 0.01, 0.01);

    // var mixedColor: vec3<f32> = u.background_color;
    // HHMMMM this works, but do I really want this computed for each fragment?
    // Maybe create a conversion func on the Bevy side instead.
    var mixedColor: vec3<f32> = bevy_color(u.background_color);


    return vec4<f32>(mixedColor, 1.0);


}

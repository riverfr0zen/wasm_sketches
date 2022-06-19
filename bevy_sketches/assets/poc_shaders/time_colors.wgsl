#import "shader_common/common_uniform.wgsl"


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};


[[group(1), binding(0)]]
var<uniform> uniform_data: CommonUniformData;


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    let freq_r = 5.0;
    let freq_g = 3.0;
    let freq_b = 1.0;

    var output_color = vec4<f32>(
        abs(sin(uniform_data.time * freq_r)), 
        abs(sin(uniform_data.time * freq_g)), 
        abs(sin(uniform_data.time * freq_b)),  
        1.0
    );
    return output_color;
}

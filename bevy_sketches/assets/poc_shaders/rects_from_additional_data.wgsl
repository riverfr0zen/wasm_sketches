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
    num_rects: u32;
};


[[group(1), binding(0)]]
var<uniform> u: CustomUniformData;


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var backgroundColor: vec3<f32> = vec3<f32>(0.01, 0.01, 0.01);
    var rectColor: vec3<f32> = vec3<f32>(abs(sin(u.common.time)) * 0.5, 0.0, 0.0);
    var rectColor2: vec3<f32> = vec3<f32>(0.0, abs(cos(u.common.time)) * 0.5, 0.0);
    var rectColor3: vec3<f32> = vec3<f32>(0.0, 0.0, abs(tan(u.common.time)) * 0.5);

    var mixedColor: vec3<f32> = backgroundColor;

    if (u.num_rects >= 1u) {
        var myrect = rect(xlate(vec2<f32>(0.25, 0.05), input.uv, 0.5, 0.25));
        mixedColor = mix(mixedColor, rectColor3, myrect);
    }

    if (u.num_rects >= 2u) {
        var myrect = rect(xlate(vec2<f32>(0.25, 0.36), input.uv, 0.5, 0.25));
        mixedColor = mix(mixedColor, rectColor2, myrect);
    }

    if (u.num_rects >= 3u) {
        var myrect = rectSoft(xlate(vec2<f32>(0.25, 0.66), input.uv, 0.5, 0.25), 0.1);
        mixedColor = mix(mixedColor, rectColor, myrect);
    }

    return vec4<f32>(mixedColor, 1.0);


}

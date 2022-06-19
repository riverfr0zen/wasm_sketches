// This shader demonstrates using the resolution from uniform data sent by the Bevy app.
//
// In truth, Bevy already normalizes coordinates to a 0.0-1.0 scale, so I'm not sure what
// use the resolution would be here (in my reading of The Book of Shaders so far, resolution is
// just used to normalize the pixel as so: `vec2 st = gl_FragCoord.xy/u_resolution;`).
//
// However, here it is, in case it's ever needed. 

#import "shader_common/common_uniform.wgsl"
#import "shader_common/shapefuncs.wgsl"


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};


[[group(1), binding(0)]]
var<uniform> u: CommonUniformData;


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var backgroundColor: vec3<f32> = vec3<f32>(0.5, 0.5, 1.0);
    var rectColor: vec3<f32> = vec3<f32>(0.5, 0.0, 0.0);
    var rectColor2: vec3<f32> = vec3<f32>(0.0, 0.5, 0.0);
    var rectColor3: vec3<f32> = vec3<f32>(0.0, 0.0, 0.5);

    var mixedColor: vec3<f32> = backgroundColor;

    var myrect = rect(xlate(vec2<f32>(0.0, 0.1), input.uv, 0.25, 0.25));
    mixedColor = mix(mixedColor, rectColor, myrect);

    // Creating the second rect using resolution data send from Bevy. Doing some
    // conversions to normalize to 0-1 scale. If resolution data is being sent
    // correctly, then the second (green) rect dimensions should be half 
    // (see the `2.0` divisor) of the resolution.
    var w_from_res = (u.resolution.x / 2.0) / u.resolution.x;
    var h_from_res = (u.resolution.y / 2.0) / u.resolution.y;
    var myrect2 = rect(xlate(vec2<f32>(0.0, 0.4), input.uv, w_from_res, h_from_res));
    mixedColor = mix(mixedColor, rectColor2, myrect2);

    return vec4<f32>(mixedColor, 1.0);


}

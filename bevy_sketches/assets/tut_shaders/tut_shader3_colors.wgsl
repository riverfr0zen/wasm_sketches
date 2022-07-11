// Exploring colors from https://thebookofshaders.com/06/
//
// The VertexOutput info below is coming from existing Bevy machinery, I believe.
// See: https://github.com/bevyengine/bevy/blob/main/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
// See also: https://youtu.be/EKS0SSq8UPQ?t=613

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct Time {
    [[location(0)]] time: f32;
};

[[group(1), binding(0)]]
var<uniform> time: Time;



[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var yellow: vec3<f32>;
    yellow.r = 1.0;
    yellow.y = 1.0;
    yellow[2] = 0.0;
    // return vec4<f32>(yellow, 1.0);

    // Swap green / blue to create magenta
    var magenta: vec3<f32> = yellow.rbg;
    // return vec4<f32>(magenta, 1.0);

    // Swap to create green
    var green: vec3<f32> = yellow.bgb;
    return vec4<f32>(green, 1.0);
}

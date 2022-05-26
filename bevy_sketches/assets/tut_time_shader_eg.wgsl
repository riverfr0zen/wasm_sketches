// This is the shader being used with the time example code in this Discord thread:
// https://discord.com/channels/691052431525675048/742884593551802431/970382134663577661
//
// Apart from being an example about using time, it also shows how to do 
// the vertex shader (vertex stage) part (i.e. not just the fragment shader stage as I had
// been working with up to now).

#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct BottomColor {
    [[location(0)]] color: vec4<f32>;
};

struct TopColor {
    [[location(0)]] color: vec4<f32>;
};

struct Time {
    [[location(0)]] time: f32;
};

struct VertexIn {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOut {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] uv: vec2<f32>;
};

[[group(1), binding(0)]]
var<uniform> bottom_color: BottomColor;
[[group(1), binding(1)]]
var<uniform> top_color: TopColor;
[[group(1), binding(2)]]
var<uniform> time: Time;

[[stage(vertex)]]
fn vertex(in: VertexIn) -> VertexOut {
    let speed = 2.0;
    let displacement = 0.5;

    var out: VertexOut;
    out.position = view.view_proj
        * vec4<f32>(
            in.position.x + sin(time.time * speed) * displacement * in.uv.y,
            in.position.y,
            in.position.z,
            1.0
        );
    out.uv = in.uv;
    return out;
}

[[stage(fragment)]]
fn fragment(in: VertexOut) -> [[location(0)]] vec4<f32> {
    return bottom_color.color * in.uv.y + top_color.color * (1.0 - in.uv.y);
}
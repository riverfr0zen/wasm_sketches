// Exploring line shaping from https://thebookofshaders.com/05/
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


fn plot(uv: vec2<f32>, pct: f32) -> f32 {
    // return smoothStep(pct - 0.02, pct, uv.y) - smoothStep(pct, pct + 0.02, uv.y);
    return smoothStep(pct - 0.02, pct, 1.0-uv.y) - smoothStep(pct, pct + 0.02, 1.0-uv.y);

    // Playing with the feather(?) of the line using time
    // let fade = (time.time % 10.0) / 10.0;
    // return smoothStep(pct - fade, pct, 1.0-uv.y) - smoothStep(pct, pct + fade, 1.0-uv.y);
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var colorA: vec3<f32> = vec3<f32>(1.0, 0.0, 0.0);
    var colorB: vec3<f32> = vec3<f32>(0.0, 0.0, 1.0);

    // var mixedColor = mix(colorA, colorB, 0.5);
    // return vec4<f32>(mixedColor, 1.0);

    // var pct: f32 = abs(sin(time.time));
    // var mixedColor = mix(colorA, colorB, pct);
    // return vec4<f32>(mixedColor, 1.0);



}

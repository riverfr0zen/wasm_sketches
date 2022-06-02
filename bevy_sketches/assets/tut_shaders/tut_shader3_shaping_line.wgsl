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


fn plot(uv: vec2<f32>) -> f32 {
    return smoothStep(0.01, 0.0, abs(uv.y - uv.x));
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {

    // `y` is apparently a common symbol for brightness (or luminance)
    // var y: f32 = input.uv.x;
    var y: f32 = input.uv.x;

    var color = vec3<f32>(y);

    var pct: f32 = plot(input.uv);
    color = (1.0-pct)*color+pct*vec3<f32>(0.0,1.0,0.0);

    return vec4<f32>(color, 1.0);

}

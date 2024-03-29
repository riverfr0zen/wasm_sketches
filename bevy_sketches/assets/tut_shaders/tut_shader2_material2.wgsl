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
    let freq_r = 5.0;
    let freq_g = 3.0;
    let freq_b = 1.0;

    // var output_color = vec4<f32>(abs(sin(time.time)), 0.0, 0.0, 1.0);
    // var output_color = vec4<f32>(abs(sin(time.time * freq)), 0.0, 0.0, 1.0);
    var output_color = vec4<f32>(
        abs(sin(time.time * freq_r)), 
        abs(sin(time.time * freq_g)), 
        abs(sin(time.time * freq_b)),  
        1.0
    );
    return output_color;
}

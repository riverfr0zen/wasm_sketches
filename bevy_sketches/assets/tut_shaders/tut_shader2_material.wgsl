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
    let alpha: f32 = (time.time % 5.0) / 5.0;
    let xpos: f32 = input.uv.x * 800.0;
    if (
        xpos < 50.0 || 
        (xpos > 100.0 && xpos < 200.0) || 
        (xpos > 400.0 && xpos < 600.0) ||
        (xpos > 700.0 && xpos < 800.0)
    ) {
        var output_color = vec4<f32>(input.uv.x, input.uv.y, 0.0, alpha);
        return output_color;
    } else {
        var output_color = vec4<f32>(input.uv.y, input.uv.x, 0.0, alpha);
        return output_color;
    }

    // var output_color = vec4<f32>(input.uv,0.0,1.0);
    // return output_color;
}

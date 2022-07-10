// Exploring sin/cos from https://thebookofshaders.com/05/
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


fn plot2(uv: vec2<f32>, pct: f32) -> f32 {
    // return smoothStep(pct - 0.02, pct, uv.y) - smoothStep(pct, pct + 0.02, uv.y);
    return smoothStep(pct - 0.02, pct, 1.0-uv.y) - smoothStep(pct, pct + 0.02, 1.0-uv.y);

    // Playing with the feather(?) of the line using time
    // let fade = (time.time % 10.0) / 10.0;
    // return smoothStep(pct - fade, pct, 1.0-uv.y) - smoothStep(pct, pct + fade, 1.0-uv.y);
}


// Divide by 2.0 to scale down y coordinates since display coord system does not have "negative coordinates".
// After scaling down, compensate for half the wave being in negative y coords by adding 0.5
// and thus pushing the full sine wave upwards
fn adjustedSin(x: f32) -> f32 {
    // return sin(x) / 2.0 + 0.5;
    return sin(x) / 20.0 + 0.5;
}



[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {

    // var y: f32 = sin(input.uv.x);
    // var y: f32 = sin(input.uv.x * 20.0);
    // var y: f32 = abs(sin(input.uv.x * 20.0));
    // var y = adjustedSin(input.uv.x * 20.0);
    // Messing w/ time
    // var y: f32 = adjustedSin(input.uv.x * 20.0 + time.time);
    // var y: f32 = adjustedSin(input.uv.x * 20.0 + (time.time * 10.0));
    // var y: f32 = adjustedSin(input.uv.x + time.time);
    // Interesting effects when multiplying x with time. 
    // Might want to restart the app to reset time -- but the later effects are interesting too.
    // var y: f32 = adjustedSin(input.uv.x * 20.0 * time.time + (time.time * 10.0));
    // var y: f32 = adjustedSin(input.uv.x * time.time);

    // var y: f32 = adjustedSin(input.uv.x * abs(sin(time.time)) * 10.0);
    // var y: f32 = adjustedSin(input.uv.x * (time.time % 10.0) + time.time);
    // var y: f32 = adjustedSin(input.uv.x * sin(time.time % 50.0) + time.time);
    var y: f32 = adjustedSin(input.uv.x * 10.0 + time.time);
 
    var color = vec3<f32>(y);

    // var pct: f32 = plot(input.uv);
    var pct: f32 = plot2(input.uv, y);

    // Exploring progressively how we get to the final color
    color = pct * vec3<f32>(0.0,1.0,0.0);
    // color = color + pct * vec3<f32>(0.0,1.0,0.0);
    // color = (1.0-pct) * color + pct * vec3<f32>(0.0,1.0,0.0);

    return vec4<f32>(color, 1.0);

}

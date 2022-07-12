#import "shader_common/common_uniform.wgsl"
//#import "shader_common/shapefuncs.wgsl"


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};


[[group(1), binding(0)]]
var<uniform> u: CommonUniformData;


fn plot2(uv: vec2<f32>, pct: f32, top_feather: f32, bottom_feather: f32) -> f32 {
    // return smoothStep(pct - 0.02, pct, uv.y) - smoothStep(pct, pct + 0.02, uv.y);
    return smoothStep(pct - bottom_feather, pct, 1.0-uv.y) - smoothStep(pct, pct + top_feather, 1.0-uv.y);

    // Playing with the feather of the line using time
    // let fade = (u.time % 10.0) / 10.0;
    // return smoothStep(pct - fade, pct, 1.0-uv.y) - smoothStep(pct, pct + fade, 1.0-uv.y);
}


// Divide by 2.0 to scale down y coordinates since display coord system does not have "negative coordinates".
// After scaling down, compensate for half the wave being in negative y coords by adding 0.5
// and thus pushing the full sine wave upwards
fn adjusted_sin(x: f32) -> f32 {
    // return sin(x) / 2.0 + 0.5;
    return sin(x) / 20.0 + 0.5;
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var backgroundColor: vec3<f32> = vec3<f32>(1.0, 0.65, 0.2);
    var waveColor = vec3<f32>(0.043, 0.525, 0.756);

    // var y: f32 = sin(input.uv.x);
    // var y: f32 = sin(input.uv.x * 20.0);
    // var y: f32 = abs(sin(input.uv.x * 20.0));
    // var y = adjusted_sin(input.uv.x * 20.0);
    // Messing w/ time
    // var y: f32 = adjusted_sin(input.uv.x * 20.0 + u.time);
    // var y: f32 = adjusted_sin(input.uv.x * 20.0 + (u.time * 10.0));
    // var y: f32 = adjusted_sin(input.uv.x + u.time);
    // Interesting effects when multiplying x with time. 
    // Might want to restart the app to reset time -- but the later effects are interesting too.
    // var y: f32 = adjusted_sin(input.uv.x * 20.0 * u.time + (u.time * 10.0));
    // var y: f32 = adjusted_sin(input.uv.x * u.time);

    // var y: f32 = adjusted_sin(input.uv.x * abs(sin(u.time)) * 10.0);
    // var y: f32 = adjusted_sin(input.uv.x * (u.time % 10.0) + u.time);
    // var y: f32 = adjusted_sin(input.uv.x * sin(u.time % 50.0) + u.time);

    var y: f32 = adjusted_sin(input.uv.x * abs(sin(u.time % 10.0)) * 5.0 + u.time);


    // var pct: f32 = plot2(input.uv, y, 0.02, 0.02);
    var pct: f32 = plot2(input.uv, y, 0.05, 5.0);

    // This creates a beveled effect
    // waveColor = pct * waveColor;

    waveColor = mix(backgroundColor, waveColor, pct);
    return vec4<f32>(waveColor, 1.0);


}

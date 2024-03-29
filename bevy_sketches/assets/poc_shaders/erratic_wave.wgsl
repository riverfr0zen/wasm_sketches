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

fn plot(uv: vec2<f32>, pct: f32, feather: f32) -> f32 {
    return smoothStep(pct + feather, pct, 1.0-uv.y);
}



// Divide by 2.0 to scale down y coordinates since display coord system does not have "negative coordinates".
// After scaling down, compensate for half the wave being in negative y coords by adding 0.5
// and thus pushing the full sine wave upwards
fn adjusted_sin(x: f32, y_shrink: f32, wave_height: f32) -> f32 {
    // return sin(x) / 2.0 + wave_height;
    // return sin(x) / 20.0 + wave_height;
    return sin(x) / y_shrink + wave_height;
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    // var backgroundColor: vec3<f32> = vec3<f32>(1.0, 0.65, 0.2);
    // var waveColor = vec3<f32>(0.043, 0.525, 0.756);
    var backgroundColor: vec3<f32> = vec3<f32>(1.0, 0.65, 0.2) * (input.uv.y * 4.0);
    var waveColor = vec3<f32>(0.043, 0.525, 0.756) * ((1.0-input.uv.y) * 0.8);

    // var y: f32 = adjusted_sin(input.uv.x * abs(sin(u.time % 60.0)) * 5.5 + u.time);
    let wave_height = 0.5;
    let max_y_shrink = 30.0;
    let min_y_shrink = 10.0;
    let wave_y_timeframe = u.time % max_y_shrink;
    let wave_y_timeframe2x = u.time % (wave_y_timeframe * 2.0);
    var wave_y_shrink = wave_y_timeframe;
    // Using the 2x timeframe to step `wave_y_shrink` "backwards" if we've gone past the
    // single-direction timeframe. This is a technique that can be used to get a 
    // "pendulum" or "back-and-forth" effect from time and modulus.
    if (wave_y_timeframe2x > max_y_shrink) {
        wave_y_shrink = max_y_shrink - wave_y_timeframe;
    }
    if (wave_y_shrink < min_y_shrink) {
        wave_y_shrink = min_y_shrink;
    }
    var y: f32 = adjusted_sin(input.uv.x * abs(sin(u.time % 60.0)) * 5.5 + u.time, wave_y_shrink, wave_height);
    // var y: f32 = adjusted_sin(input.uv.x + u.time, wave_y_shrink, wave_height);


    // var pct: f32 = plot2(input.uv, y, 0.02, 0.02);
    // var pct: f32 = plot2(input.uv, y, 0.05, 50.0);
    var pct: f32 = plot(input.uv, y, 0.05);

    // This creates a beveled effect
    // waveColor = pct * waveColor;

    waveColor = mix(backgroundColor, waveColor, pct);
    return vec4<f32>(waveColor, 1.0);


}

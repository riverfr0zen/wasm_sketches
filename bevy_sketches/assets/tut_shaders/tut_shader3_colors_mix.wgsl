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


fn plot(uv: vec2<f32>, pct: f32) -> f32 {
    // return smoothStep(pct - 0.02, pct, uv.y) - smoothStep(pct, pct + 0.02, uv.y);
    return smoothStep(pct - 0.01, pct, 1.0-uv.y) - smoothStep(pct, pct + 0.01, 1.0-uv.y);

    // Playing with the feather(?) of the line using time
    // let fade = (time.time % 10.0) / 10.0;
    // return smoothStep(pct - fade, pct, 1.0-uv.y) - smoothStep(pct, pct + fade, 1.0-uv.y);
}


fn plottedLinesEg(input: VertexOutput, colorA: vec3<f32>, colorB: vec3<f32>) -> vec3<f32> {
    var pct: vec3<f32> = vec3<f32>(input.uv.x);

    pct.r = smoothStep(0.0, 1.0, input.uv.x);
    pct.g = sin(input.uv.x * 3.14);
    pct.b = pow(input.uv.x, 0.5);

    var mixedColor = mix(colorA, colorB, pct);

    // Plot transition lines for each channel
    // "Remember that the lines visualize the amount of colorA and colorB to mix per channel."
    mixedColor = mix(mixedColor, vec3<f32>(1.0, 0.0, 0.0), plot(input.uv, pct.r));
    mixedColor = mix(mixedColor, vec3<f32>(0.0, 1.0, 0.0), plot(input.uv, pct.g));
    mixedColor = mix(mixedColor, vec3<f32>(0.0, 0.0, 1.0), plot(input.uv, pct.b));

    return mixedColor;
}


fn animPlottedLinesEg(input: VertexOutput, colorA: vec3<f32>, colorB: vec3<f32>) -> vec3<f32> {
    var pct: vec3<f32> = vec3<f32>(input.uv.x);

    pct.r = smoothStep(
        0.0 + ((time.time % 10.0) / 10.0), 
        1.0 - ((time.time % 10.0) / 10.0), 
        input.uv.x
    );
    pct.g = sin(input.uv.x * 3.14 * abs(sin(time.time)));
    pct.b = pow(input.uv.x, (time.time % 10.0) / 10.0);

    var mixedColor = mix(colorA, colorB, pct);

    // Plot transition lines for each channel
    // "Remember that the lines visualize the amount of colorA and colorB to mix per channel."
    mixedColor = mix(mixedColor, vec3<f32>(1.0, 0.0, 0.0), plot(input.uv, pct.r));
    mixedColor = mix(mixedColor, vec3<f32>(0.0, 1.0, 0.0), plot(input.uv, pct.g));
    mixedColor = mix(mixedColor, vec3<f32>(0.0, 0.0, 1.0), plot(input.uv, pct.b));

    return mixedColor;
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

    // var mixedColor = plottedLinesEg(input, colorA, colorB);
    // return vec4<f32>(mixedColor, 1.0);

    var mixedColor = animPlottedLinesEg(input, colorA, colorB);
    return vec4<f32>(mixedColor, 1.0);

}

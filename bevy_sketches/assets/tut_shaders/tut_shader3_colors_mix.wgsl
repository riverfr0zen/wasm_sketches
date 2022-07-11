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


// Adapted from https://thebookofshaders.com/edit.php?log=160509131554
fn rect(uv: vec2<f32>, size: vec2<f32>) -> f32 {
	var size = 0.25 - size * 0.25;
    var area: vec2<f32> = step(size, uv * (1.0 - uv));
	return area.x*area.y;
}


fn featheredRect(uv: vec2<f32>, size: vec2<f32>, feather: f32) -> f32 {
	var size = 0.25 - size * 0.25;
    var area: vec2<f32> = smoothStep(size - feather, size, uv * (1.0 - uv));
	return area.x*area.y;
}


fn drawCross(input: VertexOutput, colorA: vec3<f32>, colorB: vec3<f32>) -> vec3<f32> {
    var mixedColor = mix(colorA, colorB, input.uv.y);
    var rectColor: vec3<f32> = vec3<f32>(0.15, 0.30, 0.15);
    var rectColor2: vec3<f32> = vec3<f32>(0.9, 0.30, 0.15);
    mixedColor = mix(mixedColor, rectColor, rect(input.uv, vec2<f32>(0.1, 0.9)));
    mixedColor = mix(mixedColor, rectColor2, rect(input.uv, vec2<f32>(0.9, 0.1)));
    return mixedColor;
}


fn drawCrossAnim(input: VertexOutput, colorA: vec3<f32>, colorB: vec3<f32>) -> vec3<f32> {
    var mixedColor = mix(colorA, colorB, input.uv.y);
    var rectColor: vec3<f32> = vec3<f32>(0.15, 0.30, 0.15);
    var rectColor2: vec3<f32> = vec3<f32>(0.9, 0.30, 0.15);
    if (time.time % 2.0 > 1.0) {
        mixedColor = mix(mixedColor, rectColor, rect(input.uv, vec2<f32>(0.1, 0.9)));
        mixedColor = mix(mixedColor, rectColor2, rect(input.uv, vec2<f32>(0.9, 0.1)));
    } else {
        mixedColor = mix(mixedColor, rectColor, rect(input.uv, vec2<f32>(0.9, 0.1)));
        mixedColor = mix(mixedColor, rectColor2, rect(input.uv, vec2<f32>(0.1, 0.9)));
    }
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

    // var mixedColor = animPlottedLinesEg(input, colorA, colorB);
    // return vec4<f32>(mixedColor, 1.0);

    // var mixedColor = drawCross(input, colorA, colorB);
    // return vec4<f32>(mixedColor, 1.0);

    // var mixedColor = mix(colorA, colorB, input.uv.y);
    // var rectColor: vec3<f32> = vec3<f32>(0.15, 0.30, 0.15);
    // mixedColor = mix(mixedColor, rectColor, rect(input.uv, vec2<f32>(0.1, 0.9)));
    // return vec4<f32>(mixedColor, 1.0);

    // var mixedColor = drawCross(input, colorA, colorB);
    // return vec4<f32>(mixedColor, 1.0);

    // var mixedColor = drawCrossAnim(input, colorA, colorB);
    // return vec4<f32>(mixedColor, 1.0);

    var mixedColor = mix(colorA, colorB, input.uv.y);
    var rectColor: vec3<f32> = vec3<f32>(0.15, 0.30, 0.15);
    mixedColor = mix(mixedColor, rectColor, featheredRect(input.uv, vec2<f32>(0.1, 0.5), 0.02));
    return vec4<f32>(mixedColor, 1.0);

}

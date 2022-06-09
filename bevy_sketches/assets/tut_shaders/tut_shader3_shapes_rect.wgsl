// Exploring shapes from https://thebookofshaders.com/07/
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


fn rect(uv: vec2<f32>, width: f32, height: f32) -> vec3<f32> {
    // Since our params for the rect are width/height instead of top/left/bottom/right padding,
    // we derive the padding values (edge value for the step function calls) from the given 
    // width/height.
    var wEdge: f32 = (1.0 - width) / 2.0;
    var hEdge: f32 = (1.0 - height) / 2.0;

    // var left: f32 = step(wEdge, uv.x);
    // var top: f32 = step(hEdge, uv.y);
    // var right: f32 = step(wEdge, 1.0-uv.x);
    // var bottom: f32 = step(hEdge, 1.0-uv.y);
    // return vec3<f32>(left * bottom * top * right);

    // Save a few lines by passing in two values to step, as shown at:
    // https://thebookofshaders.com/07/
    var topLeft = step(vec2<f32>(wEdge, hEdge), uv);
    var bottomRight = step(vec2<f32>(wEdge, hEdge), 1.0-uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}


fn translatedRect(uv: vec2<f32>, position: vec2<f32>, width: f32, height: f32) -> vec3<f32> {
    var uv = uv + 0.5 - vec2<f32>(width / 2.0, height / 2.0);
    uv = uv - position;
    return rect(uv, width, height);
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(800.0, 800.0);
    var backgroundColor: vec3<f32> = vec3<f32>(0.5, 0.5, 1.0);
    var rectColor: vec3<f32> = vec3<f32>(0.5, 0.0, 0.0);
    var rectColor2: vec3<f32> = vec3<f32>(0.0, 0.5, 0.0);
    var rectColor3: vec3<f32> = vec3<f32>(0.0, 0.0, 0.5);


    // var left: f32 = step(0.2, input.uv.x);
    // var top: f32 = step(0.2, input.uv.y);
    // var right: f32 = step(0.2, 1.0-input.uv.x);
    // var bottom: f32 = step(0.2, 1.0-input.uv.y);

    // var mixedColor: vec3<f32>;
    // mixedColor = vec3<f32>(left * bottom * top * right);
    // return vec4<f32>(mixedColor, 1.0);

    // var mixedColor: vec3<f32>;
    // var myrect = vec3<f32>(left * bottom * top * right);
    // mixedColor = mix(backgroundColor, rectColor, myrect);
    // return vec4<f32>(mixedColor, 1.0);

    // var mixedColor: vec3<f32>;
    // var myrect = rect(input.uv, 0.8, 0.8);
    // mixedColor = mix(backgroundColor, rectColor, myrect);
    // return vec4<f32>(mixedColor, 1.0);

    var mixedColor: vec3<f32>;
    var myrect = rect(input.uv, 0.8, 0.8);
    mixedColor = mix(backgroundColor, rectColor, myrect);
    var myrect2 = translatedRect(input.uv, vec2<f32>(0.0, 0.0), 0.4, 0.4);
    mixedColor = mix(mixedColor, rectColor2, myrect2);
    var myrect3 = translatedRect(input.uv, vec2<f32>(0.35, 0.65), 0.6, 0.3);
    mixedColor = mix(mixedColor, rectColor3, myrect3);
    return vec4<f32>(mixedColor, 1.0);
}

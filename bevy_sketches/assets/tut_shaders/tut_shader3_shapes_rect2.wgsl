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


struct ShapeTranslation {
    uv: vec2<f32>;
    width: f32;
    height: f32;
};


// Adapted from https://thebookofshaders.com/07/
fn rect(uv: vec2<f32>, width: f32, height: f32) -> vec3<f32> {
    var wEdge: f32 = (1.0 - width) / 2.0;
    var hEdge: f32 = (1.0 - height) / 2.0;

    var topLeft = step(vec2<f32>(wEdge, hEdge), uv);
    var bottomRight = step(vec2<f32>(wEdge, hEdge), 1.0-uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}

fn rectFeathered(uv: vec2<f32>, width: f32, height: f32, feather: f32) -> vec3<f32> {
    var wEdge: f32 = (1.0 - width) / 2.0;
    var hEdge: f32 = (1.0 - height) / 2.0;

    var topLeft = smoothStep(vec2<f32>(wEdge, hEdge) - feather, vec2<f32>(wEdge, hEdge), uv);
    var bottomRight = smoothStep(vec2<f32>(wEdge, hEdge) - feather, vec2<f32>(wEdge, hEdge), 1.0-uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}


fn translate(position: vec2<f32>, uv: vec2<f32>, width: f32, height: f32) -> vec2<f32> {
    // First move coordinates to 0 (adjusting for rect width/height)
    var uv = uv + 0.5 - vec2<f32>(width / 2.0, height / 2.0);
    // Move to requested position
    uv = uv - position;
    return uv;
}

fn translatedRect(position: vec2<f32>, uv: vec2<f32>, width: f32, height: f32) -> vec3<f32> {
    // First move coordinates to 0 (adjusting for rect width/height)
    var uv = uv + 0.5 - vec2<f32>(width / 2.0, height / 2.0);
    // Move to requested position
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


    // var mixedColor: vec3<f32>;
    // var myrect = rect(input.uv, 0.8, 0.8);
    // mixedColor = mix(backgroundColor, rectColor, myrect);
    // var myrect2 = translatedRect(vec2<f32>(0.0, 0.0), input.uv, 0.4, 0.4);
    // mixedColor = mix(mixedColor, rectColor2, myrect2);
    // var myrect3 = translatedRect(vec2<f32>(0.35, 0.65), input.uv, 0.6, 0.3);
    // mixedColor = mix(mixedColor, rectColor3, myrect3);
    // return vec4<f32>(mixedColor, 1.0);

    var mixedColor: vec3<f32>;
    var myrect = rectFeathered(input.uv, 0.2, 0.2, 0.1);
    mixedColor = mix(backgroundColor, rectColor, myrect);
    var myrect2 = rectFeathered(translate(vec2<f32>(0.0, 0.00), input.uv, 0.5, 0.3), 0.5, 0.3, 0.1);
    mixedColor = mix(mixedColor, rectColor2, myrect2);
    return vec4<f32>(mixedColor, 1.0);


}

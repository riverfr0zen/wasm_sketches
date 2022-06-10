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
    position: vec2<f32>;
    uv: vec2<f32>;
    width: f32;
    height: f32;
};

struct ShapeBasics {
    uv: vec2<f32>;
    width: f32;
    height: f32;
};

// Adapted from https://thebookofshaders.com/07/
fn rect(shape: ShapeBasics) -> vec3<f32> {
    var wEdge: f32 = (1.0 - shape.width) / 2.0;
    var hEdge: f32 = (1.0 - shape.height) / 2.0;

    var topLeft = step(vec2<f32>(wEdge, hEdge), shape.uv);
    var bottomRight = step(vec2<f32>(wEdge, hEdge), 1.0-shape.uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}

fn rectFeathered(shape: ShapeBasics, feather: f32) -> vec3<f32> {
    var wEdge: f32 = (1.0 - shape.width) / 2.0;
    var hEdge: f32 = (1.0 - shape.height) / 2.0;

    var topLeft = smoothStep(vec2<f32>(wEdge, hEdge) - feather, vec2<f32>(wEdge, hEdge), shape.uv);
    var bottomRight = smoothStep(vec2<f32>(wEdge, hEdge) - feather, vec2<f32>(wEdge, hEdge), 1.0-shape.uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}


fn xlate(translation: ShapeTranslation) -> ShapeBasics {
    // First move coordinates to 0 (adjusting for rect width/height)
    var shapeBasics = ShapeBasics( 
        translation.uv + 0.5 - vec2<f32>(translation.width / 2.0, translation.height / 2.0),
        translation.width,
        translation.height
    );

    // Move to requested position
    shapeBasics.uv = shapeBasics.uv - translation.position;
    return shapeBasics;
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
    var myrect = rect(ShapeBasics(input.uv, 0.2, 0.2));
    mixedColor = mix(backgroundColor, rectColor, myrect);
    var myrect2 = rectFeathered(xlate(ShapeTranslation(vec2<f32>(0.0, 0.00), input.uv, 0.5, 0.3)), 0.1);
    mixedColor = mix(mixedColor, rectColor2, myrect2);
    return vec4<f32>(mixedColor, 1.0);


}

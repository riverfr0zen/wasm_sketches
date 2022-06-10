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


struct ShapeBasics {
    uv: vec2<f32>;
    width: f32;
    height: f32;
};


// See tut_shader3_shapes_rect.wgsl for how I arrived at this function
fn rect(shape: ShapeBasics) -> vec3<f32> {
    var wEdge: f32 = (1.0 - shape.width) / 2.0;
    var hEdge: f32 = (1.0 - shape.height) / 2.0;

    var topLeft = step(vec2<f32>(wEdge, hEdge), shape.uv);
    var bottomRight = step(vec2<f32>(wEdge, hEdge), 1.0-shape.uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}

// Rect outline only. Basically this impl. calculates a smaller inner rect and subtracts 
// that from the shape.
fn rectOutline(shape: ShapeBasics, border: f32) -> vec3<f32> {
    var border = 1.0 - border;
    var wEdge: f32 = (1.0 - shape.width) / 2.0;
    var hEdge: f32 = (1.0 - shape.height) / 2.0;

    var wInnerEdge: f32 = (1.0 - shape.width * border) / 2.0;
    var hInnerEdge: f32 = (1.0 - shape.height * border) / 2.0;

    var topLeft = step(vec2<f32>(wEdge, hEdge), shape.uv);
    var bottomRight = step(vec2<f32>(wEdge, hEdge), 1.0-shape.uv);

    var topLeftInner = step(vec2<f32>(wInnerEdge, hInnerEdge), shape.uv);
    var bottomRightInner = step(vec2<f32>(wInnerEdge, hInnerEdge), 1.0-shape.uv);

    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y) - 
        vec3<f32>(topLeftInner.x * topLeftInner.y * bottomRightInner.x * bottomRightInner.y);
    // return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}


// Rect with soft edges (feathered)
fn rectSoft(shape: ShapeBasics, feather: f32) -> vec3<f32> {
    var wEdge: f32 = (1.0 - shape.width) / 2.0;
    var hEdge: f32 = (1.0 - shape.height) / 2.0;

    var topLeft = smoothStep(vec2<f32>(wEdge, hEdge) - feather, vec2<f32>(wEdge, hEdge), shape.uv);
    var bottomRight = smoothStep(vec2<f32>(wEdge, hEdge) - feather, vec2<f32>(wEdge, hEdge), 1.0-shape.uv);
    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y);
}


// An attempt at a universal translate function at least for basic shapes.
fn xlate(position: vec2<f32>, uv: vec2<f32>, width: f32, height: f32) -> ShapeBasics {
    // First move coordinates to 0 (adjusting for rect width/height)
    var shapeBasics = ShapeBasics( 
        uv + 0.5 - vec2<f32>(width / 2.0, height / 2.0),
        width,
        height
    );

    // Move to requested position
    shapeBasics.uv = shapeBasics.uv - position;
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
    var myrect = rect(ShapeBasics(input.uv, 0.6, 0.6));
    mixedColor = mix(backgroundColor, rectColor, myrect);
    var myrect2 = rectSoft(xlate(vec2<f32>(0.0, 0.0), input.uv, 0.5, 0.3), 0.1);
    mixedColor = mix(mixedColor, rectColor2, myrect2);
    var myrect3 = rectSoft(xlate(vec2<f32>(0.35, 0.7), input.uv, 0.5, 0.2), 0.5);
    mixedColor = mix(mixedColor, rectColor3, myrect3);
    var myrect4 = rectOutline(xlate(vec2<f32>(0.1, 0.4), input.uv, 0.5, 0.3), 0.1);
    mixedColor = mix(mixedColor, rectColor2 + rectColor, myrect4);
    var myrect4 = rectOutline(xlate(vec2<f32>(0.25, 0.5), input.uv, 0.2, 0.1), 0.05);
    mixedColor = mix(mixedColor, rectColor2 + rectColor, myrect4);
    return vec4<f32>(mixedColor, 1.0);


}

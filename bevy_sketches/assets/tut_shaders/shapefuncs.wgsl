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
}


fn rectOutlineSoft(shape: ShapeBasics, border: f32, outerFeather: f32, innerFeather: f32) -> vec3<f32> {
    var border = 1.0 - border;
    var wEdge: f32 = (1.0 - shape.width) / 2.0;
    var hEdge: f32 = (1.0 - shape.height) / 2.0;

    var wInnerEdge: f32 = (1.0 - shape.width * border) / 2.0;
    var hInnerEdge: f32 = (1.0 - shape.height * border) / 2.0;

    var topLeft = smoothStep(vec2<f32>(wEdge, hEdge) - outerFeather, vec2<f32>(wEdge, hEdge), shape.uv);
    var bottomRight = smoothStep(vec2<f32>(wEdge, hEdge) - outerFeather, vec2<f32>(wEdge, hEdge), 1.0-shape.uv);

    // var topLeftInner = step(vec2<f32>(wInnerEdge, hInnerEdge), shape.uv);
    var topLeftInner = smoothStep(
        vec2<f32>(wInnerEdge, hInnerEdge), 
        vec2<f32>(wInnerEdge, hInnerEdge) + innerFeather, 
        shape.uv
    );
    // var bottomRightInner = step(vec2<f32>(wInnerEdge, hInnerEdge), 1.0-shape.uv);
    var bottomRightInner = smoothStep(
        vec2<f32>(wInnerEdge, hInnerEdge), 
        vec2<f32>(wInnerEdge, hInnerEdge) + innerFeather, 
        1.0-shape.uv
    );


    return vec3<f32>(topLeft.x * topLeft.y * bottomRight.x * bottomRight.y) - 
        vec3<f32>(topLeftInner.x * topLeftInner.y * bottomRightInner.x * bottomRightInner.y);
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

// Bevy preprocessor supports importing (yaay!) See:
// https://bevyengine.org/news/bevy-0-6/#shader-imports
#import "tut_shaders/shapefuncs.wgsl"


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


fn manualPlacement(input: VertexOutput, backgroundColor: vec3<f32>, lightColor: vec3<f32>) -> vec3<f32> {
    var outColor: vec3<f32> = backgroundColor;

    var window = rectSoft(xlate(vec2<f32>(0.05, 0.05), input.uv, 0.05, 0.05), 0.005);
    outColor = mix(outColor, lightColor, window);

    var window = rectSoft(xlate(vec2<f32>(0.15, 0.05), input.uv, 0.05, 0.05), 0.005);
    outColor = mix(outColor, lightColor, window);

    var window = rectSoft(xlate(vec2<f32>(0.25, 0.05), input.uv, 0.05, 0.05), 0.005);
    outColor = mix(outColor, lightColor, window);

    return outColor;
}


fn grid(
    input: VertexOutput, backgroundColor: vec3<f32>, lightColor: vec3<f32>, 
    perRow: f32, perCol: f32
) -> vec3<f32> {
    var outColor: vec3<f32> = backgroundColor;
    var windowWidth = 1.0 / (perRow * 2.0);
    var windowHeight = 1.0 / (perCol * 2.0);

    for (var i: f32 = 0.0; i < 1.0; i = i + windowWidth * 2.0) {
        for (var j: f32 = 0.0; j < 1.0; j = j + windowHeight * 2.0) {
            var window = rectSoft(
                xlate(vec2<f32>(i + windowWidth * 0.5, j + windowHeight * 0.5), 
                input.uv, windowWidth, windowHeight), 
                0.005
            );
            outColor = mix(outColor, lightColor, window);
        }
    }
    return outColor;
}


fn gridFlicker(
    input: VertexOutput, backgroundColor: vec3<f32>, lightColor: vec3<f32>, 
    perRow: f32, perCol: f32
) -> vec3<f32> {
    var outColor: vec3<f32> = backgroundColor;
    var windowWidth = 1.0 / (perRow * 2.0);
    var windowHeight = 1.0 / (perCol * 2.0);

    for (var i: f32 = 0.0; i < 1.0; i = i + windowWidth * 2.0) {
        for (var j: f32 = 0.0; j < 1.0; j = j + windowHeight * 2.0) {
            var window = rectSoft(
                xlate(vec2<f32>(i + windowWidth * 0.5, j + windowHeight * 0.5), 
                input.uv, windowWidth, windowHeight), 
                0.005
            );
            outColor = mix(outColor, lightColor + sin((i + j) * time.time % 30.0), window);
        }
    }
    return outColor;
}



[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(800.0, 800.0);
    var backgroundColor: vec3<f32> = vec3<f32>(0.008, 0.01, 0.0);
    var lightColor: vec3<f32> = vec3<f32>(0.9, 1.0, 0.4);


    // var outColor = manualPlacement(input, backgroundColor, lightColor);

    var outColor = grid(input, backgroundColor, lightColor, 5.0, 3.0);
    // var outColor = grid(input, backgroundColor, lightColor, 20.0, 20.0);

    // Fun animated scaling 
    // var perRow: f32 = abs(sin(time.time) * 50.0);
    // // var perRow: f32 = ceil(abs(sin(time.time)) * 50.0);
    // var perCol: f32 = abs(sin(time.time) * 50.0);
    // // var perCol: f32 = ceil(abs(sin(time.time)) * 20.0);
    // var outColor = grid(input, backgroundColor, lightColor, perRow, perCol);


    var outColor = gridFlicker(input, backgroundColor, lightColor, 10.0, 10.0);


    return vec4<f32>(outColor, 1.0);


}

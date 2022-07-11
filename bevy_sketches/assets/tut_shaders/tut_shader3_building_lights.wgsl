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


fn rand(x: f32) -> f32 {
    return fract(sin(x) * 1.0);    
}


fn gridFlicker(
    input: VertexOutput, backgroundColor: vec3<f32>, lightColors: array<vec3<f32>, 5>, 
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
                0.025
            );
            var colorIndex = 4.0;
            // var colorIndex = ceil(time.time) % 5.0;

            // // Horizontal scroll
            // var colorIndex = floor(abs(sin(time.time) - i) * 5.0) % 5.0;
            // // Vertical scroll
            // var colorIndex = floor(abs(sin(time.time) - j) * 5.0) % 5.0;

            // Couple of interesting pattern effects
            var lightingSpeed: f32 = 0.5;
            // var colorIndex = floor(((j/j - i/j) * (time.time * lightingSpeed))) % 5.0;
            // var colorIndex = floor(((i/j - j/i) * (time.time * lightingSpeed))) % 5.0;
            // var colorIndex = floor(((15.0-j/i - j/i) * (time.time * lightingSpeed))) % 5.0;

            // Rand
            // var colorIndex = floor(rand(time.time * (1.0-j*i)) * time.time) * 0.05 % 5.0;
            // var colorIndex = floor(rand(1.0-i) * rand(1.0-j) * time.time * lightingSpeed) % 5.0;

            // // fave
            // if (floor(rand(i) * rand(j) * 120.0) % 6.0 > 3.0) {
            // if (floor(rand(1.0-i) * rand(1.0-j) * 120.0) % 6.0 > 3.0) {
            // if (floor(rand(1.0-i) * rand(1.0-j) * time.time % 120.0) % 6.0 > 3.0) {
            if (floor(rand(1.0-i) * rand(1.0-j) * (time.time + 120.0) % 120.0) % 6.0 > 3.0) {
                colorIndex = floor(rand(1.0-i) * rand(1.0-j) * time.time * lightingSpeed) % 5.0;
            } else {
                colorIndex = 0.0;
            }

            // Moire like noise effect
            // var colorIndex = floor(rand(time.time * (1.0-input.uv.x*input.uv.y)) * time.time) % 5.0;

            if (colorIndex == 0.0) {
                outColor = mix(outColor, lightColors[0], window);
            } else if (colorIndex == 1.0) {
                outColor = mix(outColor, lightColors[1], window);
            } else if (colorIndex == 2.0) {
                outColor = mix(outColor, lightColors[2], window);
            } else if (colorIndex == 3.0) {
                outColor = mix(outColor, lightColors[3], window);
            } else {
                // outColor = mix(outColor, lightColors[3], window);
                outColor = mix(outColor, lightColors[4], window);
            }
        }
    }
    return outColor;
}



[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(800.0, 800.0);
    var backgroundColor: vec3<f32> = vec3<f32>(0.008, 0.01, 0.0);
    var lightColor: vec3<f32> = vec3<f32>(0.005, 0.006, 0.0);
    var lightColor1: vec3<f32> = vec3<f32>(0.01, 0.015, 0.00);
    var lightColor2: vec3<f32> = vec3<f32>(0.04, 0.06, 0.02);
    var lightColor3: vec3<f32> = vec3<f32>(0.08, 0.1, 0.05);
    var lightColor4: vec3<f32> = vec3<f32>(0.9, 1.0, 0.6);
    var lightColors: array<vec3<f32>, 5>;
    lightColors = array<vec3<f32>, 5>(lightColor, lightColor1, lightColor2, lightColor3, lightColor4);

    // var outColor = manualPlacement(input, backgroundColor, lightColor);

    // var outColor = grid(input, backgroundColor, lightColor, 5.0, 3.0);
    // var outColor = grid(input, backgroundColor, lightColor, 20.0, 20.0);

    // // Fun animated scaling 
    // var perRow: f32 = abs(sin(time.time) * 50.0);
    // // var perRow: f32 = ceil(abs(sin(time.time)) * 50.0);
    // var perCol: f32 = abs(sin(time.time) * 50.0);
    // // var perCol: f32 = ceil(abs(sin(time.time)) * 20.0);
    // var outColor = grid(input, backgroundColor, lightColor, perRow, perCol);


    var outColor = gridFlicker(input, backgroundColor, lightColors, 10.0, 15.0);


    return vec4<f32>(outColor, 1.0);


}

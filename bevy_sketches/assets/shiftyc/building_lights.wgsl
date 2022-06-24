/// Demonstrates a shader that receives and uses data in addition to the CommonUniformData

#import "shader_common/common_uniform.wgsl"
#import "shader_common/shapefuncs.wgsl"


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};


/// Nesting CommonUnifromData in a CustomUniformData struct
struct CustomUniformData {
    common: CommonUniformData;
    background_color: vec3<f32>;
    alpha: f32;
    rand_modifier: f32;
};


[[group(1), binding(0)]]
var<uniform> u: CustomUniformData;


fn rand(x: f32) -> f32 {
    return fract(sin(x) * u.rand_modifier);    
}


/// Returns a random integer (as a floating point value) between min/max
fn rand_int(seed: f32, min: f32, max: f32) -> f32 {
    return round(fract(sin(seed) * 1.0) * max + min);    
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
                // 0.025
                0.005
            );
            var colorIndex = 4.0;
            // var lightingSpeed: f32 = 0.5;
            var lightingSpeed: f32 = 0.1;

            // fave
            if (floor(rand(1.0-i) * rand(1.0-j) * (u.common.time + 120.0) % 120.0) % 6.0 > 3.0) {
                colorIndex = floor(rand(1.0-i) * rand(1.0-j) * u.common.time * lightingSpeed) % 5.0;
            } else {
                colorIndex = 0.0;
            }

            if (colorIndex == 0.0) {
                outColor = mix(outColor, lightColors[0], window);
            } else if (colorIndex == 1.0) {
                outColor = mix(outColor, lightColors[1], window);
            } else if (colorIndex == 2.0) {
                outColor = mix(outColor, lightColors[2], window);
            } else if (colorIndex == 3.0) {
                outColor = mix(outColor, lightColors[3], window);
            } else {
                outColor = mix(outColor, lightColors[4], window);
            }
        }
    }
    return outColor;
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var mixedColor: vec3<f32> = u.background_color;

    var lightColor: vec3<f32> = vec3<f32>(0.005, 0.006, 0.0);
    var lightColor1: vec3<f32> = vec3<f32>(0.01, 0.015, 0.00);
    var lightColor2: vec3<f32> = vec3<f32>(0.04, 0.06, 0.02);
    var lightColor3: vec3<f32> = vec3<f32>(0.08, 0.1, 0.05);
    var lightColor4: vec3<f32> = vec3<f32>(0.9, 1.0, 0.6);
    var lightColors: array<vec3<f32>, 5>;
    // lightColors = array<vec3<f32>, 5>(lightColor, lightColor1, lightColor2, lightColor3, lightColor4);
    // This arrangement works better, with lights starting black, but then first waves being the brightest
    // color (instead of a long time with darker colors first).
    lightColors = array<vec3<f32>, 5>(lightColor, lightColor4, lightColor3, lightColor2, lightColor1);

    // mixedColor = gridFlicker(input, mixedColor, lightColors, 4.0, 20.0);

    let lights_per_row = rand_int(u.rand_modifier, 2.0, 5.0);
    let lights_per_col = rand_int(u.rand_modifier, 10.0, 20.0);
    mixedColor = gridFlicker(
        input, mixedColor, lightColors, 
        lights_per_row,
        lights_per_col,
    );

    return vec4<f32>(mixedColor, u.alpha);


}

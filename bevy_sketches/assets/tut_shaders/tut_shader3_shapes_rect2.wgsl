// Exploring shapes from https://thebookofshaders.com/07/
//
// The VertexOutput info below is coming from existing Bevy machinery, I believe.
// See: https://github.com/bevyengine/bevy/blob/main/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
// See also: https://youtu.be/EKS0SSq8UPQ?t=613


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


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(800.0, 800.0);
    var backgroundColor: vec3<f32> = vec3<f32>(0.5, 0.5, 1.0);
    var rectColor: vec3<f32> = vec3<f32>(0.5, 0.0, 0.0);
    var rectColor2: vec3<f32> = vec3<f32>(0.0, 0.5, 0.0);
    var rectColor3: vec3<f32> = vec3<f32>(0.0, 0.0, 0.5);

    var mixedColor: vec3<f32> = backgroundColor;

    var myrect = rect(ShapeBasics(input.uv, 0.6, 0.6));
    mixedColor = mix(mixedColor, rectColor, myrect);

    var myrect2 = rectSoft(xlate(vec2<f32>(0.0, 0.0), input.uv, 0.5, 0.3), 0.1);
    mixedColor = mix(mixedColor, rectColor2, myrect2);

    var myrect3 = rectSoft(xlate(vec2<f32>(0.35, 0.7), input.uv, 0.5, 0.2), 0.5);
    mixedColor = mix(mixedColor, rectColor3, myrect3);

    var myrect4 = rectOutline(xlate(vec2<f32>(0.1, 0.4), input.uv, 0.5, 0.3), 0.1);
    mixedColor = mix(mixedColor, rectColor + rectColor3, myrect4);

    var myrect5 = rectOutline(xlate(vec2<f32>(0.25, 0.5), input.uv, 0.2, 0.1), 0.05);
    mixedColor = mix(mixedColor, rectColor + rectColor3, myrect5);

    var myrect6 = rectOutlineSoft(xlate(vec2<f32>(0.65, 0.3), input.uv, 0.3, 0.25), 0.5, 0.2, 0.02);
    mixedColor = mix(mixedColor, rectColor + rectColor2, myrect6);

    return vec4<f32>(mixedColor, 1.0);


}

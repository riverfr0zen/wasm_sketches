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


// Adapted from https://thebookofshaders.com/edit.php?log=160509131554
fn featheredRect(uv: vec2<f32>, size: vec2<f32>, feather: f32) -> f32 {
	var size = 0.25 - size * 0.25;
    var area: vec2<f32> = smoothStep(size - feather, size, uv * (1.0 - uv));
	return area.x*area.y;

}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var resolution = vec2<f32>(800.0, 800.0);
    var buildingColor: vec3<f32> = vec3<f32>(0.0, 0.01, 0.0);

    var mixedColor = buildingColor;
    var lightColor: vec3<f32> = vec3<f32>(0.9, 1.0, 0.3);
    
    // I don't understand how to position the rects wherever I want them yet
    // Will come back to this later.
    mixedColor = mix(mixedColor, lightColor, featheredRect(
        // input.uv, 
        input.uv + vec2<f32>(0.46, 0.0),
        vec2<f32>(0.002, 0.002), 
        0.0005
    ));
    mixedColor = mix(mixedColor, lightColor, featheredRect(
        input.uv + vec2<f32>(0.0, 0.0), 
        vec2<f32>(0.002, 0.002), 
        0.0005
    ));
    return vec4<f32>(mixedColor, 1.0);

}

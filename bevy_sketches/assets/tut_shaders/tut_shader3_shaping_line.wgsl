// Exploring line shaping from https://thebookofshaders.com/05/
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


fn plot(uv: vec2<f32>) -> f32 {
    // return smoothStep(0.01, 0.0, abs(uv.y - uv.x));
    // "Flip" `uv.y` to match slope in Book of Shaders
    return smoothStep(0.01, 0.0, abs(1.0-uv.y - uv.x));
}

fn plot2(uv: vec2<f32>, pct: f32) -> f32 {
    // return smoothStep(pct - 0.02, pct, uv.y) - smoothStep(pct, pct + 0.02, uv.y);
    return smoothStep(pct - 0.02, pct, 1.0-uv.y) - smoothStep(pct, pct + 0.02, 1.0-uv.y);

    // Playing with the feather(?) of the line using time
    // let fade = (time.time % 10.0) / 10.0;
    // return smoothStep(pct - fade, pct, 1.0-uv.y) - smoothStep(pct, pct + fade, 1.0-uv.y);
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {

    // `y` is apparently a common symbol for brightness (or luminance)
    // var y: f32 = input.uv.x;
    // var y: f32 = pow(input.uv.x, 5.0);
    // var y: f32 = step(0.5, input.uv.x);
    // var y: f32 = smoothStep(0.1, 0.9, input.uv.x);
    var y: f32 = smoothStep(0.2, 0.5, input.uv.x) - smoothStep(0.5, 0.8, input.uv.x);
    // Various other items from here: https://thebookofshaders.com/05/#:~:text=Some%20extra%20useful%20functions
    // var y: f32 = input.uv.x % 0.5;    
    // var y: f32 = input.uv.x % sin(time.time);    
    // var y: f32 = fract(input.uv.x * 5.0);    
    // var y: f32 = ceil(input.uv.x);    
    // var y: f32 = floor(input.uv.x);    

    var color = vec3<f32>(y);

    // var pct: f32 = plot(input.uv);
    var pct: f32 = plot2(input.uv, y);

    // Exploring progressively how we get to the final color
    // color = pct * vec3<f32>(0.0,1.0,0.0);
    // color = color + pct * vec3<f32>(0.0,1.0,0.0);
    color = (1.0-pct) * color + pct * vec3<f32>(0.0,1.0,0.0);

    return vec4<f32>(color, 1.0);

}

let BEVY_COLOR_MOD: f32 = 0.1;

// For some reason, colors coming from Bevy have to be adjusted.
// Possibly related to this discussion, but the solution in the discussion does not work.
// https://github.com/bevyengine/bevy/discussions/2783
//
// Instead, we modify the color values by 0.1, which does seem to work.
fn bevy_color(incoming_color: vec3<f32>) -> vec3<f32> {
    return incoming_color * 0.1;
}

struct CommonUniformData {
    time: f32;
    resolution: vec2<f32>;
};

// @TODO: Right now this shader is not very efficient

#import "shader_common/common_uniform.wgsl"
#import "shader_common/shapefuncs.wgsl"


struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};


[[group(1), binding(0)]]
var<uniform> u: CommonUniformData;


fn grid(
    input: VertexOutput, backgroundColor: vec3<f32>, strokeColor: vec3<f32>, 
    perRow: f32, perCol: f32
) -> vec3<f32> {
    var outColor: vec3<f32> = backgroundColor;
    var cellWidth = 1.0 / perRow;
    var cellHeight = 1.0 / perCol;

    for (var i: f32 = -0.0; i < 1.0; i = i + cellWidth) {
        for (var j: f32 = 0.0; j < 1.0; j = j + cellHeight) {
            // var window = rectOutlineSoft(
            //     xlate(
            //         vec2<f32>(i, j), 
            //         input.uv, 
            //         cellWidth, 
            //         cellHeight
            //     ), 
            //     0.01,
            //     0.005,
            //     0.005
            // );

            var window = rectOutline(
                xlate(
                    vec2<f32>(i, j), 
                    input.uv, 
                    cellWidth, 
                    cellHeight
                ), 
                0.1,
            );
            outColor = mix(outColor, strokeColor, window);
        }
    }
    return outColor;
}


[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var backgroundColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var lightColor: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);


    // // Fun animated scaling 
    // var perRow: f32 = 10.0;
    var perRow: f32 = abs(sin(u.time) * 20.0);
    // var perRow: f32 = ceil(abs(sin(u.time)) * 10.0);

    // var perCol: f32 = 10.0;
    var perCol: f32 = abs(sin(u.time) * 20.0);
    // var perCol: f32 = ceil(abs(sin(u.time)) * 10.0);

    var outColor: vec3<f32> = grid(input, backgroundColor, lightColor, perRow, perCol);

    return vec4<f32>(outColor, 1.0);


}

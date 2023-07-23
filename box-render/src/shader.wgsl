// vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
}

@group(1) @binding(0)
var<uniform> camera : CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) texture_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var output: VertexOutput;
    output.texture_coords = model.texture_coords;
    output.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return output;
}


@group(0) @binding(0) 
var t_texture: texture_2d<f32>;
@group(0) @binding(1) 
var s_texture: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_texture, s_texture, in.texture_coords);
}
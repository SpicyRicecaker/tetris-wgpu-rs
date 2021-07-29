// Vertex shader

// uniform shaders use block
[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
};
// inside the render_pipeline_layout, the idx corresponsinds to idx in group
// group(0) would be the texture_binding
[[group(1), binding(0)]]
var<uniform> uniforms: Uniforms;

// Vertex output stores the inputs and outputs of our vertex shader
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

// Marks this func as an entry point for vertex shader
[[stage(vertex)]]
fn main (
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    // out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

// Fragment shder 2
[[stage(fragment)]]
fn main_2(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(0.5, 0.5, 0.5, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
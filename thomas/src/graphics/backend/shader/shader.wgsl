// Vertex shader
struct Uniforms {
    // Matrix that transforms perspective
    view: mat4x4<f32>;
    model: mat4x4<f32>;
};

// inside the render_pipeline_layout, the idx corresponsinds to idx in group
// group(0) would be the texture_binding
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

// Vertex output stores the inputs and outputs of our vertex shader
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

// Marks this func as an entry point for vertex shader
[[stage(vertex)]]
fn vs_main (
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // x, y, z, w
    // w is always 1 unless you need mipmapping (farther objects look further away) which is useless for 2d

    // view is for camera
    // model converts screen coords from 0-[max screensize] (e.g., (400, 599)) to normalized coords from 0-1 (e.g., (0.5122123, 0.99231))
    out.clip_position = uniforms.view * uniforms.model * vec4<f32>(model.position, 1.0);
    out.color = model.color;

    return out;
}

// Fragment shader
[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
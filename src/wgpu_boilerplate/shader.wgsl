// Vertex shader

// Vertex output stores the inputs and outputs of our vertex shader
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

// Marks this func as an entry point for vertex shader
[[stage(vertex)]]
fn main (
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// Fragment shder 2
[[stage(fragment)]]
fn main_2(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.5, 0.5, 0.5, 1.0);
}
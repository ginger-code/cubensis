struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};


[[group(0), binding(0)]]
var texture_input : texture_2d<f32>;
[[group(0), binding(1)]]
var sampler_input : sampler;


[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    return out;
}


[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var tex = textureSample(texture_input, sampler_input, in.uv);
    return vec4<f32>(tex.r, tex.g, tex.b, 1.0) ;
}

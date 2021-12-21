struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};

[[block]]
struct TimeInfo {
    frame_index: u32;
    time_seconds: f32;
    frame_time_seconds: f32;
};

[[group(0), binding(0)]]
var<uniform> time_info: TimeInfo;
[[group(0), binding(1)]]
var wave_texture: texture_1d<f32>;
[[group(0), binding(2)]]
var spectrum_texture: texture_1d<f32>;
[[group(0), binding(3)]]
var audio_sampler : sampler;
[[group(1), binding(0)]]
var history_texture: texture_2d<f32>;
[[group(1), binding(1)]]
var history_sampler : sampler;



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
    var prev_color = textureSample(history_texture, history_sampler, 1.0 - in.uv ) * sin(time_info.time_seconds);
    var spectrum_value = textureSample(spectrum_texture, audio_sampler, in.uv.x ).r;
    return vec4<f32>(spectrum_value * 2.0, in.uv.x, in.uv.y, 1.0) + vec4<f32>(prev_color.grb, 0.5);
}

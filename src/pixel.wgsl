struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var result: VertexOutput;
    let xx = in_vertex_index / 2u;
    let yy = in_vertex_index & 1u;
    result.tex_coord = vec2<f32>(f32(xx), f32(yy));
    let x = f32(i32(xx) * 2 - 1);
    let y = f32(i32(yy) * 2 - 1);
    result.position = vec4<f32>(x, y, 0., 1.);
    return result;
}

@group(0)
@binding(0)
var r_color: texture_2d<u32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureLoad(r_color, vec2<i32>(vertex.tex_coord * 256.0), 0);
    let v = f32(tex.x) / 255.0;
    return vec4<f32>(1.0 - (v * 5.0), 1.0 - (v * 15.0), 1.0 - (v * 50.0), 1.0);
}
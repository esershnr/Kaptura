struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // Generate a fullscreen quad from vertex index
    let x = f32((in_vertex_index & 1u) << 2u);
    let y = f32((in_vertex_index & 2u) << 1u);
    out.tex_coords = vec2<f32>(x * 0.5, y * 0.5);
    out.clip_position = vec4<f32>(x - 1.0, 1.0 - y, 0.0, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct Uniforms {
    sharpen_amount: f32,
    _pad: f32,
    tex_width: f32,
    tex_height: f32,
};

@group(0) @binding(2)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    
    if (uniforms.sharpen_amount <= 0.0) {
        return base_color;
    }

    // Offset for 1 pixel
    let dx = 1.0 / uniforms.tex_width;
    let dy = 1.0 / uniforms.tex_height;

    // Simple 3x3 sharpening kernel (Laplacian)
    // [ 0 -1  0 ]
    // [-1  5 -1 ]
    // [ 0 -1  0 ]
    // Weight = 5 for center, -1 for neighbors
    
    let c = base_color;
    let u = textureSample(t_diffuse, s_diffuse, in.tex_coords + vec2<f32>(0.0, -dy));
    let d = textureSample(t_diffuse, s_diffuse, in.tex_coords + vec2<f32>(0.0, dy));
    let l = textureSample(t_diffuse, s_diffuse, in.tex_coords + vec2<f32>(-dx, 0.0));
    let r = textureSample(t_diffuse, s_diffuse, in.tex_coords + vec2<f32>(dx, 0.0));

    let sharpened = c * 5.0 - (u + d + l + r);
    
    // Mix original and sharpened based on amount
    let final_color = mix(base_color, sharpened, uniforms.sharpen_amount);
    
    return vec4<f32>(final_color.rgb, base_color.a);
}

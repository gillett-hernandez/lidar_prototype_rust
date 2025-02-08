#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> hue: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material_color * brightness_factor;
}

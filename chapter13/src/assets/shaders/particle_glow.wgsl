// src/assets/shaders/particle_glow.wgsl
// Custom shader for particles
// Creates a radial gradient glow effect with additive blending
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from center (UV space is 0-1)
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(mesh.uv, center) * 2.0; // *2 to normalize to 0-1
    
    // Create radial gradient
    // Bright center → fades to edges
    let radial = 1.0 - smoothstep(0.0, 1.0, dist);
    
    // Add extra glow in the center
    let glow = pow(1.0 - dist, 3.0);
    
    // Combine radial gradient with center glow
    let intensity = radial * 0.7 + glow * 0.5;
    
    // Boost brightness near center for hot glow effect
    let brightness = 1.0 + glow * 0.5;
    
    // Apply to color (supports HDR - values > 1.0)
    let final_rgb = color.rgb * brightness;
    let final_alpha = color.a * intensity;
    
    return vec4<f32>(final_rgb, final_alpha);
}
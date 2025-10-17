#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct FogMaterial {
    player_pos: vec2<f32>,
    vision_radius: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: FogMaterial;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Get distance from fragment to player position in world space
    let dist = distance(mesh.world_position.xy, material.player_pos);
    
    // Inside vision radius = transparent, outside = black
    // Use smoothstep for soft edge
    let fade_width = 40.0;
    let alpha = smoothstep(material.vision_radius - fade_width, material.vision_radius + fade_width, dist);
    
    // Return black with calculated alpha (0 = transparent inside circle, 1 = opaque outside)
    return vec4<f32>(0.0, 0.0, 0.0, alpha);
}

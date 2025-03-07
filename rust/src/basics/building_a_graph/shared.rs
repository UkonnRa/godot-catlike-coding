use godot::classes::StandardMaterial3D;
use godot::prelude::*;

/// Creates a material with the specified color
pub fn create_colored_material(color: Color) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(color);
    material
}

/// Creates a green material for the graph cubes (for simple implementations)
pub fn create_graph_material() -> Gd<StandardMaterial3D> {
    create_colored_material(Color::GREEN)
}

/// Creates a color gradient material for the graph
/// This attempts to mimic the shader effect from the tutorial
#[allow(dead_code)]
pub fn create_gradient_material() -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new_gd();

    // Use a blue-green gradient (similar to the tutorial)
    material.set_albedo(Color::from_rgb(0.0, 0.8, 0.5));

    material
}

/// Computes a color based on position, similar to the tutorial
/// This is based on the exact shader approach from the tutorial:
/// `surface.Albedo.rg = input.worldPos.xy * 0.5 + 0.5;`
pub fn compute_position_color(x: f32, y: f32) -> Color {
    // Normalize position from [-1,1] to [0,1] range
    // This exactly matches the tutorial's shader approach:
    // color.rg = position.xy * 0.5 + 0.5;
    let r = x * 0.5 + 0.5;
    let g = y * 0.5 + 0.5;

    // Ensure values are clamped to [0,1] range (equivalent to saturate() in shader)
    let r = r.clamp(0.0, 1.0);
    let g = g.clamp(0.0, 1.0);

    // Return color with r,g components from position (exactly as tutorial shows)
    Color::from_rgb(r, g, 0.0)
}

/// Computes the step size based on resolution
pub fn compute_step(resolution: i32) -> f32 {
    2.0 / resolution as f32
}

/// Computes the cube size based on step size
pub fn compute_cube_size(step: f32) -> Vector3 {
    Vector3::ONE * step
}

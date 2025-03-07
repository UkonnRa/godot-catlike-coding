use godot::classes::StandardMaterial3D;
use godot::prelude::*;

/// Type for 1D math functions (single parameter)
#[allow(dead_code)]
pub type MathFunction1D = fn(f32) -> f32;

/// Type for 2D math functions (two parameters)
#[allow(dead_code)]
pub type MathFunction2D = fn(f32, f32) -> f32;

/// Type for 3D math functions (three parameters)
#[allow(dead_code)]
pub type MathFunction3D = fn(f32, f32, f32) -> f32;

/// Available math functions for the various surfaces
pub enum FunctionName {
    Wave,
    MultiWave,
    Ripple,
    Sphere,
    Torus,
}

impl FunctionName {
    /// Get the string representation of a function name
    pub fn as_str(&self) -> &'static str {
        match self {
            FunctionName::Wave => "Wave",
            FunctionName::MultiWave => "Multi Wave",
            FunctionName::Ripple => "Ripple",
            FunctionName::Sphere => "Sphere",
            FunctionName::Torus => "Torus",
        }
    }
}

/// Simple wave function based on sine
pub fn wave(x: f32) -> f32 {
    (std::f32::consts::PI * x).sin()
}

/// Multi-wave function combining several sine waves
pub fn multi_wave(x: f32) -> f32 {
    let y1 = (std::f32::consts::PI * x).sin();
    let y2 = (2.0 * std::f32::consts::PI * x).sin() * 0.5;
    let y3 = (4.0 * std::f32::consts::PI * x).sin() * 0.25;
    y1 + y2 + y3
}

/// Ripple function creates a radial wave from the center
pub fn ripple(x: f32, z: f32) -> f32 {
    // Calculate distance from center (0,0)
    let distance = (x * x + z * z).sqrt();
    if distance < 0.0001 {
        return 1.0; // Avoid division by zero
    }
    (std::f32::consts::PI * 4.0 * distance).sin() / (1.0 + 10.0 * distance)
}

/// Sphere function for 3D surfaces
#[allow(dead_code)]
pub fn sphere(x: f32, z: f32, t: f32) -> f32 {
    // Simple sphere function - sqrt(1 - x² - z²) with animation
    let r2 = x * x + z * z;
    if r2 > 1.0 {
        return 0.0; // Outside the sphere
    }
    (1.0 - r2).sqrt() + (t * 0.5).sin() * 0.1
}

/// Torus function for 3D parametric surfaces
pub fn torus(u: f32, v: f32, t: f32) -> Vector3 {
    // Torus parameters
    let r1 = 0.7 + (t * 0.5).sin() * 0.1; // Major radius
    let r2 = 0.15 + (t * 3.0).sin() * 0.05; // Minor radius (tube radius)

    // Convert parameters to angles
    let s = u * 2.0 * std::f32::consts::PI; // 0 to 2π
    let t_angle = v * 2.0 * std::f32::consts::PI; // 0 to 2π

    // Calculate 3D position using parametric equation of a torus
    let x = (r1 + r2 * t_angle.cos()) * s.cos();
    let y = r2 * t_angle.sin();
    let z = (r1 + r2 * t_angle.cos()) * s.sin();

    Vector3::new(x, y, z)
}

/// Creates a material with the specified color
pub fn create_colored_material(color: Color) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(color);
    material
}

/// Creates a color based on position - used for visualization
pub fn compute_position_color(x: f32, y: f32, z: f32) -> Color {
    // Normalize coordinates into 0-1 range for RGB
    let r = (x + 1.0) * 0.5;
    let g = (y + 1.0) * 0.5;
    let b = (z + 1.0) * 0.5;

    // Create color with full alpha
    Color::from_rgb(r, g, b)
}

/// Compute the step size based on resolution
pub fn compute_step(resolution: i32) -> f32 {
    2.0 / resolution as f32
}

/// Compute the cube size based on step
pub fn compute_cube_size(step: f32) -> Vector3 {
    Vector3::new(step, step, step) * 0.8 // Make slightly smaller to leave gaps
}

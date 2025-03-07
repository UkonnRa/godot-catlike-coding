use godot::classes::{CsgBox3D, StandardMaterial3D};
use godot::prelude::*;

/// Type for mathematical functions that operate on one parameter (2D)
pub type MathFunction1D = fn(f32) -> f32;

/// Type for mathematical functions that operate on two parameters (3D)
pub type MathFunction2D = fn(f32, f32) -> f32;

/// Type for mathematical functions that operate on three parameters (4D)
pub type MathFunction3D = fn(f32, f32, f32) -> f32;

/// Function names for the UI
pub enum FunctionName {
    Wave,
    MultiWave,
    Ripple,
    Sphere,
    Torus,
}

impl FunctionName {
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

// Function Library

/// Sine wave function - f(x) = sin(π * x)
pub fn wave(x: f32) -> f32 {
    (std::f32::consts::PI * x).sin()
}

/// Multi-wave function - combination of sine waves
pub fn multi_wave(x: f32) -> f32 {
    let y1 = (std::f32::consts::PI * x).sin();
    let y2 = (2.0 * std::f32::consts::PI * x).sin() * 0.5;
    let y3 = (4.0 * std::f32::consts::PI * x).sin() * 0.25;
    y1 + y2 + y3
}

/// Ripple function - circular wave from the center
pub fn ripple(x: f32, z: f32) -> f32 {
    let d = (x * x + z * z).sqrt();
    if d != 0.0 {
        (std::f32::consts::PI * 4.0 * d).sin() / (1.0 + 10.0 * d)
    } else {
        1.0
    }
}

/// Sphere function - creates a spherical surface
pub fn sphere(x: f32, z: f32, t: f32) -> f32 {
    let r = 0.9 + (std::f32::consts::PI * t * 0.5).sin() * 0.1;
    let d = (x * x + z * z).sqrt();
    if d < r { (r * r - d * d).sqrt() } else { 0.0 }
}

/// Torus function - creates a torus (donut) shape
pub fn torus(u: f32, v: f32, t: f32) -> Vector3 {
    let r1 = 0.7 + (std::f32::consts::PI * t * 0.5).sin() * 0.1;
    let r2 = 0.15 + (std::f32::consts::PI * t * 2.0).sin() * 0.05;

    // Convert parameter range [0,1] to [0,2π]
    let s = u * std::f32::consts::TAU;
    let t = v * std::f32::consts::TAU;

    // Compute point on torus
    let x = (r1 + r2 * s.cos()) * t.cos();
    let y = r2 * s.sin();
    let z = (r1 + r2 * s.cos()) * t.sin();

    Vector3::new(x, y, z)
}

/// Creates a colored material for a point
pub fn create_colored_material(color: Color) -> Gd<StandardMaterial3D> {
    let mut material = StandardMaterial3D::new_gd();
    material.set_albedo(color);
    material
}

/// Computes a color based on position
pub fn compute_position_color(x: f32, y: f32, z: f32) -> Color {
    // Normalize position from [-1,1] to [0,1] range
    let nx = (x + 1.0) * 0.5;
    let ny = (y + 1.0) * 0.5;
    let nz = (z + 1.0) * 0.5;

    // Create color from position
    Color::from_rgb(nx, ny, nz)
}

/// Computes the step size based on resolution
pub fn compute_step(resolution: i32) -> f32 {
    2.0 / resolution as f32
}

/// Computes the cube size based on step size
pub fn compute_cube_size(step: f32) -> Vector3 {
    Vector3::ONE * step * 0.95 // Slightly smaller to leave gaps
}

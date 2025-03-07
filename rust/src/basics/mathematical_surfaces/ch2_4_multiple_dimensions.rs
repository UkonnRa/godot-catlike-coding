use godot::classes::{CsgBox3D, Node3D, StandardMaterial3D, Time};
use godot::prelude::*;

use crate::basics::mathematical_surfaces::shared;
use crate::basics::mathematical_surfaces::shared::FunctionName;

/// MathSurface component for Chapter 4 - Multiple Dimensions
/// This implementation supports 3D functions for spheres and tori
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch24MathSurface {
    /// Resolution determines the number of points on each axis
    #[export]
    #[init(val = 20)]
    resolution: i32,

    /// Current function to display
    #[export]
    #[init(val = 0)]
    function_index: i32,

    /// Transition value (0-1) for blending between current function and next
    #[export]
    #[init(val = 0.0)]
    transition: f32,

    /// Animation speed multiplier
    #[export]
    #[init(val = 1.0)]
    animation_speed: f32,

    /// Track points for animation
    points: Vec<Gd<CsgBox3D>>,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Ch24MathSurface {
    fn ready(&mut self) {
        let step = shared::compute_step(self.resolution);
        let size = shared::compute_cube_size(step * 0.8); // Slightly smaller for aesthetics
        let function_name = self.get_function_name();

        // Create a grid of points in the XZ plane
        for i in 0..self.resolution {
            for j in 0..self.resolution {
                let mut cube = CsgBox3D::new_alloc();
                cube.set_size(size);

                // Get grid position
                let grid_size = 2.0; // Size of the grid in world units
                let u = (i as f32 / (self.resolution - 1) as f32) * grid_size - 1.0;
                let v = (j as f32 / (self.resolution - 1) as f32) * grid_size - 1.0;

                // Initialize with the current function
                let pos = self.calculate_position(u, v, 0.0);

                cube.set_position(pos);

                // Set color based on position
                let color = shared::compute_position_color(pos.x, pos.y, pos.z);
                let material = shared::create_colored_material(color);
                cube.set_material(&material);

                self.base_mut().add_child(&cube);
                self.points.push(cube);
            }
        }

        godot_print!(
            "Created Ch24MathSurface with {} x {} = {} points using function: {}",
            self.resolution,
            self.resolution,
            self.resolution * self.resolution,
            function_name.as_str()
        );
    }

    fn process(&mut self, _delta: f64) {
        let time = Time::singleton().get_ticks_msec() as f32 / 1000.0 * self.animation_speed;
        let func_index = self.function_index; // Store function index before loops

        // Calculate the next function for transition (wrapping if needed)
        let next_func_index = if self.transition > 0.0 {
            (func_index + 1) % 5
        } else {
            func_index
        };

        // Calculate grid parameters
        let grid_size = 2.0; // Size of the grid in world units
        let res = self.resolution;

        // Update each point in the grid
        for i in 0..res {
            for j in 0..res {
                let idx = (i * res + j) as usize;
                if idx >= self.points.len() {
                    continue; // Safety check
                }

                // Calculate UV coordinates
                let u = (i as f32 / (res - 1) as f32) * grid_size - 1.0;
                let v = (j as f32 / (res - 1) as f32) * grid_size - 1.0;

                // Calculate positions for both current and next function
                let current_pos = match func_index {
                    0 => {
                        // Wave function (1D) - only uses u
                        let y = shared::wave(u + time);
                        Vector3::new(u, y, v)
                    }
                    1 => {
                        // Multi-wave function (1D) - only uses u
                        let y = shared::multi_wave(u + time);
                        Vector3::new(u, y, v)
                    }
                    2 => {
                        // Ripple function (2D) - uses both u and v
                        let y = shared::ripple(u, v + time);
                        Vector3::new(u, y, v)
                    }
                    3 => {
                        // Sphere function (3D)
                        // Convert from UV coordinates to spherical coordinates
                        let r = 0.9 + (shared::wave(u + time) * 0.1); // Radius with wave
                        let s = u * std::f32::consts::PI; // Azimuthal angle (0 to pi)
                        let t = v * 2.0 * std::f32::consts::PI; // Polar angle (0 to 2pi)

                        // Convert spherical to Cartesian
                        let x = r * s.sin() * t.cos();
                        let y = r * s.cos();
                        let z = r * s.sin() * t.sin();

                        Vector3::new(x, y, z)
                    }
                    4 => {
                        // Torus function
                        shared::torus(u, v, time)
                    }
                    _ => {
                        // Default to wave
                        let y = shared::wave(u + time);
                        Vector3::new(u, y, v)
                    }
                };

                // Position for the next function
                let next_pos = match next_func_index {
                    0 => {
                        let y = shared::wave(u + time);
                        Vector3::new(u, y, v)
                    }
                    1 => {
                        let y = shared::multi_wave(u + time);
                        Vector3::new(u, y, v)
                    }
                    2 => {
                        let y = shared::ripple(u, v + time);
                        Vector3::new(u, y, v)
                    }
                    3 => {
                        let r = 0.9 + (shared::wave(u + time) * 0.1);
                        let s = u * std::f32::consts::PI;
                        let t = v * 2.0 * std::f32::consts::PI;
                        let x = r * s.sin() * t.cos();
                        let y = r * s.cos();
                        let z = r * s.sin() * t.sin();
                        Vector3::new(x, y, z)
                    }
                    4 => shared::torus(u, v, time),
                    _ => {
                        let y = shared::wave(u + time);
                        Vector3::new(u, y, v)
                    }
                };

                // Blend between current and next position
                let blend_pos = if self.transition > 0.0 {
                    Vector3::new(
                        current_pos.x * (1.0 - self.transition) + next_pos.x * self.transition,
                        current_pos.y * (1.0 - self.transition) + next_pos.y * self.transition,
                        current_pos.z * (1.0 - self.transition) + next_pos.z * self.transition,
                    )
                } else {
                    current_pos
                };

                // Update cube position
                let cube = &mut self.points[idx];
                cube.set_position(blend_pos);

                // Update color based on position
                let color = shared::compute_position_color(blend_pos.x, blend_pos.y, blend_pos.z);
                if let Some(material) = cube.get_material() {
                    if let Ok(mut std_material) = material.try_cast::<StandardMaterial3D>() {
                        std_material.set_albedo(color);
                    }
                }
            }
        }
    }
}

impl Ch24MathSurface {
    /// Get the current function name
    fn get_function_name(&self) -> FunctionName {
        match self.function_index {
            0 => FunctionName::Wave,
            1 => FunctionName::MultiWave,
            2 => FunctionName::Ripple,
            3 => FunctionName::Sphere,
            4 => FunctionName::Torus,
            _ => FunctionName::Wave,
        }
    }

    /// Calculate position based on function type, u, v coordinates and time
    fn calculate_position(&self, u: f32, v: f32, t: f32) -> Vector3 {
        match self.function_index {
            0 => {
                // Wave function (1D) - only uses u
                let y = shared::wave(u + t);
                Vector3::new(u, y, v)
            }
            1 => {
                // Multi-wave function (1D) - only uses u
                let y = shared::multi_wave(u + t);
                Vector3::new(u, y, v)
            }
            2 => {
                // Ripple function (2D) - uses both u and v
                let y = shared::ripple(u, v + t);
                Vector3::new(u, y, v)
            }
            3 => {
                // Sphere function (3D)
                // Convert from UV coordinates to spherical coordinates
                let r = 0.9 + (shared::wave(u + t) * 0.1); // Radius with wave
                let s = u * std::f32::consts::PI; // Azimuthal angle (0 to pi)
                let t = v * 2.0 * std::f32::consts::PI; // Polar angle (0 to 2pi)

                // Convert spherical to Cartesian
                let x = r * s.sin() * t.cos();
                let y = r * s.cos();
                let z = r * s.sin() * t.sin();

                Vector3::new(x, y, z)
            }
            4 => {
                // Torus function

                shared::torus(u, v, t)
            }
            _ => {
                // Default to wave
                let y = shared::wave(u + t);
                Vector3::new(u, y, v)
            }
        }
    }
}

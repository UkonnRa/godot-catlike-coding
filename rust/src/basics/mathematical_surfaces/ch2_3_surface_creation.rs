use godot::classes::{CsgBox3D, Node3D, StandardMaterial3D, Time};
use godot::prelude::*;

use crate::basics::mathematical_surfaces::shared;
use crate::basics::mathematical_surfaces::shared::FunctionName;

/// MathSurface component for Chapter 3 - Surface Creation
/// This implementation creates a 2D grid of points to form a surface
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch23MathSurface {
    /// Resolution determines the number of points on each axis
    #[export]
    #[init(val = 20)]
    resolution: i32,

    /// Current function to display
    #[export]
    #[init(val = 0)]
    function_index: i32,

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
impl INode3D for Ch23MathSurface {
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
                let x = i as f32 * step - 1.0 + (step * 0.5);
                let z = j as f32 * step - 1.0 + (step * 0.5);

                // Initialize with the current function
                let y = match self.function_index {
                    0 => shared::wave(x),
                    1 => shared::multi_wave(x),
                    2 => shared::ripple(x, z),
                    _ => shared::wave(x),
                };

                cube.set_position(Vector3::new(x, y, z));

                // Set color based on position
                let color = shared::compute_position_color(x, y, z);
                let material = shared::create_colored_material(color);
                cube.set_material(&material);

                self.base_mut().add_child(&cube);
                self.points.push(cube);
            }
        }

        godot_print!(
            "Created Ch23MathSurface with {} x {} = {} points using function: {}",
            self.resolution,
            self.resolution,
            self.resolution * self.resolution,
            function_name.as_str()
        );
    }

    fn process(&mut self, _delta: f64) {
        let time = Time::singleton().get_ticks_msec() as f32 / 1000.0 * self.animation_speed;
        let func_index = self.function_index; // Store function index before loops

        // Separate out the resolutions for easier calculations
        let res = self.resolution;

        // Update each point in the grid
        for i in 0..res {
            for j in 0..res {
                let idx = (i * res + j) as usize;
                if idx >= self.points.len() {
                    continue; // Safety check
                }

                let cube = &mut self.points[idx];
                let pos = cube.get_position();
                let x = pos.x;
                let z = pos.z;

                // Calculate based on current function
                let y = match func_index {
                    0 => shared::wave(x + time),
                    1 => shared::multi_wave(x + time),
                    2 => shared::ripple(x, z + time),
                    _ => shared::wave(x + time),
                };

                // Update position
                cube.set_position(Vector3::new(x, y, z));

                // Update color
                let color = shared::compute_position_color(x, y, z);
                if let Some(material) = cube.get_material() {
                    if let Ok(mut std_material) = material.try_cast::<StandardMaterial3D>() {
                        std_material.set_albedo(color);
                    }
                }
            }
        }
    }
}

impl Ch23MathSurface {
    /// Get the current function name
    fn get_function_name(&self) -> FunctionName {
        match self.function_index {
            0 => FunctionName::Wave,
            1 => FunctionName::MultiWave,
            2 => FunctionName::Ripple,
            _ => FunctionName::Wave,
        }
    }
}

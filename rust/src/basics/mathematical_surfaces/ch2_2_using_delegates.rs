use godot::classes::{CsgBox3D, Node3D, StandardMaterial3D, Time};
use godot::prelude::*;

use crate::basics::mathematical_surfaces::shared;
use crate::basics::mathematical_surfaces::shared::{FunctionName, MathFunction1D, MathFunction2D};

/// MathSurface component for Chapter 2 - Using Delegates
/// This implementation works with function delegates
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch22MathSurface {
    /// Resolution determines the number of points on the line
    #[export]
    #[init(val = 40)]
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
impl INode3D for Ch22MathSurface {
    fn ready(&mut self) {
        let step = shared::compute_step(self.resolution);
        let size = shared::compute_cube_size(step);
        let function_name = self.get_function_name();

        // Create a line of cubes along the X axis
        for i in 0..self.resolution {
            let mut cube = CsgBox3D::new_alloc();
            cube.set_size(size);

            // Position the cube along the x-axis
            let x = i as f32 * step - 1.0 + (step * 0.5);

            // Initialize with the default function
            let y = match self.function_index {
                0 => shared::wave(x),
                1 => shared::multi_wave(x),
                2 => shared::ripple(x, 0.0),
                _ => shared::wave(x),
            };

            cube.set_position(Vector3::new(x, y, 0.0));

            // Set color based on position
            let color = shared::compute_position_color(x, y, 0.0);
            let material = shared::create_colored_material(color);
            cube.set_material(&material);

            self.base_mut().add_child(&cube);
            self.points.push(cube);
        }

        godot_print!(
            "Created Ch22MathSurface with {} points using function: {}",
            self.resolution,
            function_name.as_str()
        );
    }

    fn process(&mut self, _delta: f64) {
        let time = Time::singleton().get_ticks_msec() as f32 / 1000.0 * self.animation_speed;
        let func_index = self.function_index; // Store function index before loop

        // Update each point in the line
        for (_i, cube) in self.points.iter_mut().enumerate() {
            // Get current position
            let position = cube.get_position();

            // The X position doesn't change
            let x = position.x;

            // Calculate new Y position using the selected function
            let p = x + time; // Add time for animation
            let y = match func_index {
                0 => shared::wave(p),
                1 => shared::multi_wave(p),
                2 => shared::ripple(p, 0.0),
                _ => shared::wave(p),
            };

            // Update the position
            cube.set_position(Vector3::new(x, y, 0.0));

            // Update the color based on current position
            let color = shared::compute_position_color(x, y, 0.0);
            if let Some(material) = cube.get_material() {
                if let Ok(mut std_material) = material.try_cast::<StandardMaterial3D>() {
                    std_material.set_albedo(color);
                }
            }
        }
    }
}

impl Ch22MathSurface {
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

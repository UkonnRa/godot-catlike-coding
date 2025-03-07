use godot::classes::{CsgBox3D, Node3D, StandardMaterial3D, Time};
use godot::prelude::*;

use crate::basics::mathematical_surfaces::shared;
use crate::basics::mathematical_surfaces::shared::FunctionName;

/// MathSurface component for Chapter 1 - Creating a Function Library
/// This implementation builds on the Graph from the previous tutorial
/// but adds the ability to switch between different functions
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch21MathSurface {
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
impl INode3D for Ch21MathSurface {
    fn ready(&mut self) {
        let step = shared::compute_step(self.resolution);
        let size = shared::compute_cube_size(step * 0.9); // Slightly smaller to leave gaps

        // Create a 1D array of points along the x axis
        for i in 0..self.resolution {
            let mut cube = CsgBox3D::new_alloc();
            cube.set_size(size);

            // Create a colored material
            let material = shared::create_colored_material(Color::GREEN);
            cube.set_material(&material);

            // Position is just along the x-axis initially (y will be set in process)
            let x = i as f32 * step - 1.0 + (step * 0.5);
            cube.set_position(Vector3::new(x, 0.0, 0.0));

            self.base_mut().add_child(&cube);
            self.points.push(cube);
        }
    }

    fn process(&mut self, _delta: f64) {
        // Get the current time
        let time = Time::singleton();
        let t = time.get_ticks_msec() as f32 / 1000.0 * self.animation_speed;

        // Get the current function index before we start iterating
        let function_type = self.get_function_name();

        // Update all points in the graph with the current function
        for (i, cube) in self.points.iter_mut().enumerate() {
            let x = (i as f32 / (self.resolution - 1) as f32) * 2.0 - 1.0;

            // Get y position based on the current function
            let y = match function_type {
                FunctionName::Wave => shared::wave(x + t),
                FunctionName::MultiWave => shared::multi_wave(x + t),
                FunctionName::Ripple => shared::ripple(x, 0.0), // Only uses x for ripple
                FunctionName::Sphere => 0.0,                    // Not implemented in 1D
                FunctionName::Torus => 0.0,                     // Not implemented in 1D
            };

            // Update the cube position
            cube.set_position(Vector3::new(x, y, 0.0));

            // Update material color based on position
            if let Some(material) = cube.get_material() {
                let color = shared::compute_position_color(x, y, 0.0);
                if let Ok(mut std_material) = material.try_cast::<StandardMaterial3D>() {
                    std_material.set_albedo(color);
                }
            }
        }
    }
}

impl Ch21MathSurface {
    /// Get the current function name based on the function_index
    fn get_function_name(&self) -> FunctionName {
        match self.function_index {
            0 => FunctionName::Wave,
            1 => FunctionName::MultiWave,
            2 => FunctionName::Ripple,
            3 => FunctionName::Sphere,
            4 => FunctionName::Torus,
            _ => FunctionName::Wave, // Default to wave
        }
    }
}

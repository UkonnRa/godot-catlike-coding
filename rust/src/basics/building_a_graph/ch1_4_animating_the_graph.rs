use godot::classes::{CsgBox3D, Node3D, Time};
use godot::prelude::*;

use crate::basics::building_a_graph::shared;

/// The final Graph implementation from the Building a Graph tutorial
/// We create a 1D array of points along the X axis and animate their Y values
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Graph {
    /// Resolution determines the number of points on the line
    #[export]
    #[init(val = 100)] // Number of points on the line
    resolution: i32,

    /// Animation speed multiplier
    #[export]
    #[init(val = 1.0)]
    animation_speed: f32,

    /// Whether to use per-point materials to speed up rendering
    #[export]
    #[init(val = false)]
    use_shared_materials: bool,

    /// Track points for animation
    points: Vec<Gd<CsgBox3D>>,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Graph {
    fn ready(&mut self) {
        let step = self.step();
        let size = self.size();

        // Create a line of cubes along the X axis (1D, not 2D!)
        for i in 0..self.resolution {
            let mut cube = CsgBox3D::new_alloc();
            cube.set_size(size);

            // Position cube along X axis
            let x = i as f32 * step - 1.0 + (step * 0.5); // Center point in segment
            cube.set_position(Vector3::new(x, 0.0, 0.0));

            // If we're using shared materials, create them once and assign them to points
            // This is more efficient but doesn't show the tutorial's approach
            if self.use_shared_materials {
                let material =
                    shared::create_colored_material(shared::compute_position_color(x, 0.0));
                cube.set_material(&material);
            }

            self.base_mut().add_child(&cube);
            self.points.push(cube);
        }

        godot_print!("Created line graph with {} points", self.resolution);
    }

    fn process(&mut self, _delta: f64) {
        let time = Time::singleton().get_ticks_msec() as f32 / 1000.0 * self.animation_speed;

        // Update each point in the line
        for (i, cube) in self.points.iter_mut().enumerate() {
            // Get current position
            let mut position = cube.get_position();

            // Calculate X position (doesn't change)
            let x = position.x;

            // Calculate new Y position using sine wave, exactly as in the tutorial:
            // position.y = Mathf.Sin(Mathf.PI * (position.x + time));
            position.y = (std::f32::consts::PI * (x + time)).sin();

            // Update the position
            cube.set_position(position);

            // Update the color based on current position
            // This follows the tutorial's approach precisely, using world position
            if !self.use_shared_materials {
                let color = shared::compute_position_color(x, position.y);

                // Create a new material for this specific cube
                let material = shared::create_colored_material(color);
                cube.set_material(&material);
            }
        }
    }
}

impl Graph {
    /// Calculate step size based on resolution
    fn step(&self) -> f32 {
        2.0 / self.resolution as f32
    }

    /// Calculate cube size based on step
    fn size(&self) -> Vector3 {
        Vector3::ONE * self.step() * 0.9 // Slightly smaller than step to leave gaps
    }
}

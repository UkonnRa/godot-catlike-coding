use godot::classes::{CsgBox3D, Node3D};
use godot::prelude::*;

use crate::basics::building_a_graph::shared;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch12Graph {
    /// Resolution determines the number of points on the line
    #[export]
    #[init(val = 10)] // Using small resolution for this chapter
    resolution: i32,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Ch12Graph {
    fn ready(&mut self) {
        let step = shared::compute_step(self.resolution);
        let size = shared::compute_cube_size(step * 0.9); // Slightly smaller to leave gaps

        // Create a line of cubes along the x-axis with custom colors
        for i in 0..self.resolution {
            let mut cube = CsgBox3D::new_alloc();
            cube.set_size(size);

            // Position the cube along the x-axis
            let x = i as f32 * step - 1.0 + (step * 0.5);

            // For this chapter, we're using y = x² (quadratic function)
            let y = x * x;

            // Position the cube
            cube.set_position(Vector3::new(x, y, 0.0));

            // Create a material with color based on position
            // This follows the tutorial's coloring approach:
            // surface.Albedo.rg = input.worldPos.xy * 0.5 + 0.5;
            let color = shared::compute_position_color(x, y);
            let material = shared::create_colored_material(color);
            cube.set_material(&material);

            self.base_mut().add_child(&cube);
        }
    }
}

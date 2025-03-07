use godot::classes::{CsgBox3D, Node3D};
use godot::prelude::*;

use crate::basics::building_a_graph::shared;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch11Graph {
    /// Resolution determines the number of points on the line
    #[export]
    #[init(val = 10)] // Lower resolution for first example
    resolution: i32,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Ch11Graph {
    fn ready(&mut self) {
        let step = shared::compute_step(self.resolution);
        let size = shared::compute_cube_size(step * 0.9); // Slightly smaller to leave gaps
        let material = shared::create_graph_material();

        // Create a line of cubes along the x-axis (as per tutorial)
        for i in 0..self.resolution {
            let mut cube = CsgBox3D::new_alloc();
            cube.set_size(size);
            cube.set_material(&material);

            // Position the cube along the x-axis at the center of each step
            let x = i as f32 * step - 1.0 + (step * 0.5);
            cube.set_position(Vector3::new(x, 0.0, 0.0));

            self.base_mut().add_child(&cube);
        }
    }
}

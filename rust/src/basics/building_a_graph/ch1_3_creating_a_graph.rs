use godot::classes::{CsgBox3D, Node3D};
use godot::prelude::*;

use crate::basics::building_a_graph::shared;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Ch13Graph {
    /// Resolution determines the number of points on the line
    #[export]
    #[init(val = 20)] // Higher resolution for a smoother graph
    resolution: i32,

    /// Array to store references to the cubes for easy access
    points: Vec<Gd<CsgBox3D>>,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Ch13Graph {
    fn ready(&mut self) {
        let step = shared::compute_step(self.resolution);
        let size = shared::compute_cube_size(step * 0.9); // Slightly smaller to leave gaps
        let material = shared::create_graph_material();

        // Create a line of cubes along the X axis (not a grid!)
        for i in 0..self.resolution {
            let mut cube = CsgBox3D::new_alloc();
            cube.set_size(size);
            cube.set_material(&material);

            // Position the cube along the x-axis
            let x = i as f32 * step - 1.0 + (step * 0.5);

            // Calculate y position using a mathematical function
            let y = self.function(x);

            // Set the position for this point (only X and Y are used - this is a 2D graph)
            cube.set_position(Vector3::new(x, y, 0.0));

            self.base_mut().add_child(&cube);
            self.points.push(cube);
        }
    }
}

impl Ch13Graph {
    // Mathematical function to create our graph
    fn function(&self, x: f32) -> f32 {
        // Simple cubic function
        x * x * x
    }
}

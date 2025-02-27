use godot::classes::{CsgBox3D, Node3D, StandardMaterial3D, Time};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
struct Graph {
    #[export]
    #[init(val = 200)]
    resolution: i32,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for Graph {
    fn ready(&mut self) {
        let size = self.size();
        let material = Graph::material();

        for _ in 0..(self.resolution * self.resolution) {
            let mut node = CsgBox3D::new_alloc();
            node.set_size(size);
            node.set_material(&material);
            self.base_mut().add_child(&node);
        }
    }

    fn process(&mut self, _delta: f64) {
        let time = Time::singleton().get_ticks_msec() as f32 / 1000.0;

        for i in 0..self.base().get_child_count() {
            let mut node = match self
                .base()
                .get_child(i)
                .and_then(|n| n.try_cast::<CsgBox3D>().ok())
            {
                Some(node) => node,
                None => continue,
            };

            let (u, v) = (i % self.resolution, i / self.resolution);
            let (x, z) = (u as f32 * self.step() - 1.0, v as f32 * self.step() - 1.0);
            node.set_position(Vector3::new(x, 2.0 * (time + x + z).sin(), z));
        }
    }
}

impl Graph {
    fn step(&self) -> f32 {
        2.0 / self.resolution as f32
    }

    fn size(&self) -> Vector3 {
        Vector3::ONE * self.step()
    }

    fn material() -> Gd<StandardMaterial3D> {
        let mut material = StandardMaterial3D::new_gd();
        material.set_albedo(Color::GREEN);
        material
    }
}

use godot::classes::{InputEvent, InputEventKey, Node3D};
use godot::prelude::*;

/// Controller for the Basic Graph demo
/// Allows adjusting animation speed
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct GraphController {
    /// Reference to the graph
    graph: Option<Gd<Node3D>>,

    /// Animation speed levels
    speed_levels: Vec<f32>,

    /// Current speed index
    current_speed_index: usize,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for GraphController {
    fn ready(&mut self) {
        // Initialize speed levels
        self.speed_levels = vec![0.0, 0.5, 1.0, 2.0, 5.0];
        self.current_speed_index = 2; // Start at 1.0 (index 2)

        // Get reference to graph
        self.graph = Some(self.base().get_node_as::<Node3D>("Graph"));

        // Set initial animation speed
        self.set_animation_speed(self.speed_levels[self.current_speed_index]);

        godot_print!("GraphController ready - press SPACE to adjust animation speed");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed() {
                // Check if space key was pressed
                let key = key_event.get_keycode();
                // Convert key to debug string and check if it contains "Space"
                if format!("{:?}", key).contains("Space") {
                    // Cycle through animation speeds
                    self.current_speed_index =
                        (self.current_speed_index + 1) % self.speed_levels.len();
                    let speed = self.speed_levels[self.current_speed_index];
                    self.set_animation_speed(speed);
                    godot_print!("Animation speed set to: {}", speed);
                }
            }
        }
    }
}

#[godot_api]
impl GraphController {
    /// Set the animation speed for the graph
    fn set_animation_speed(&self, speed: f32) {
        if let Some(ref graph) = self.graph {
            let mut graph_mut = graph.clone();
            let speed_variant = Variant::from(speed);
            graph_mut.set("animation_speed", &speed_variant);
        }
    }
}

use godot::classes::{Button, Control, Node};
use godot::prelude::*;

/// Demo launcher controller that manages the main menu
/// Handles switching between different demo scenes
#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct DemoLauncher {
    /// Paths to the different demo scenes
    scene_paths: [GString; 3],

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl DemoLauncher {
    #[func]
    fn _ready(&mut self) {
        // Initialize scene paths
        self.scene_paths = [
            "res://Scenes/V2Graph/BasicGraph.tscn".into(),
            "res://Scenes/MathematicalSurface.tscn".into(),
            "res://Scenes/ParametricSurfaces.tscn".into(),
        ];

        // Connect signals - skip connecting in code, let the editor handle it
        godot_print!("DemoLauncher ready - select a demo to begin");
    }

    /// Handler for graph button press - this will be connected in the editor
    #[func]
    fn on_graph_button_pressed(&self) {
        self.change_scene(0);
    }

    /// Handler for math surface button press - this will be connected in the editor
    #[func]
    fn on_math_button_pressed(&self) {
        self.change_scene(1);
    }

    /// Handler for multi surface button press - this will be connected in the editor
    #[func]
    fn on_multi_button_pressed(&self) {
        self.change_scene(2);
    }

    /// Change to the selected scene
    fn change_scene(&self, index: usize) {
        if index < self.scene_paths.len() {
            godot_print!("Changing to scene: {}", self.scene_paths[index]);

            // Just print a message for now - we'll rely on the Godot editor to connect the signals
            // and the actual scene loading will happen in GDScript
            godot_print!("Please connect these buttons to GDScript functions in the editor");
            godot_print!(
                "that call get_tree().change_scene_to_file({});",
                self.scene_paths[index]
            );
        }
    }
}

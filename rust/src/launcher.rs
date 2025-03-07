use godot::classes::{Control, IControl};
use godot::global::Error;
use godot::prelude::*;

/// Demo launcher controller that manages the main menu
/// Handles switching between different demo scenes
#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct DemoLauncher {
    /// Paths to the different demo scenes
    scene_paths: [GString; 4],

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl IControl for DemoLauncher {
    fn ready(&mut self) {
        // Initialize scene paths
        self.scene_paths = [
            "res://Scenes/V2Graph/BasicGraph.tscn".into(),
            "res://Scenes/MathematicalSurface.tscn".into(),
            "res://Scenes/ParametricSurfaces.tscn".into(),
            "res://Scenes/PerformanceTester.tscn".into(),
        ];

        // Connect signals - skip connecting in code, let the editor handle it
        godot_print!("DemoLauncher ready - select a demo to begin");
    }
}

#[godot_api]
impl DemoLauncher {
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

    /// Handler for performance button press - this will be connected in the editor
    #[func]
    fn on_performance_button_pressed(&self) {
        self.change_scene(3);
    }

    /// Change to the selected scene using Godot's scene tree API
    fn change_scene(&self, index: usize) {
        if index < self.scene_paths.len() {
            let scene_path = &self.scene_paths[index];
            godot_print!("Changing to scene: {:?}", scene_path);

            // Get the scene tree and change to the selected scene
            if let Some(tree) = self.base().get_tree() {
                // Clone the tree to get a mutable instance
                let mut tree_clone = tree.clone();
                let error = tree_clone.change_scene_to_file(scene_path);
                if error == Error::OK {
                    godot_print!("Successfully changed to scene: {}", scene_path);
                } else {
                    godot_print!("Failed to change scene: {:?}", error);
                }
            } else {
                godot_print!("Failed to get scene tree!");
            }
        }
    }
}

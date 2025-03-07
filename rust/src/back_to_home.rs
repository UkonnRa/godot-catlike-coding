use godot::classes::{Control, IControl};
use godot::global::Error;
use godot::prelude::*;

/// Back to Home button controller
/// Provides navigation back to the main demo launcher
#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct BackToHome {
    /// Path to the launcher scene
    launcher_scene: GString,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl IControl for BackToHome {
    fn ready(&mut self) {
        // Set the launcher scene path
        self.launcher_scene = "res://Scenes/DemoLauncher.tscn".into();

        // Signal connections are defined in the Godot scene file
        godot_print!("BackToHome ready - button will navigate to launcher scene");
    }
}

#[godot_api]
impl BackToHome {
    /// Handler for button press - navigates back to launcher
    #[func]
    fn on_home_button_pressed(&self) {
        godot_print!("Returning to home scene: {}", self.launcher_scene);

        // Get the scene tree and change to the launcher scene
        if let Some(tree) = self.base().get_tree() {
            // Clone the tree to get a mutable instance
            let mut tree_clone = tree.clone();
            let error = tree_clone.change_scene_to_file(&self.launcher_scene);
            if error == Error::OK {
                godot_print!("Successfully changed to launcher scene");
            } else {
                godot_print!("Failed to change scene: {:?}", error);
            }
        } else {
            godot_print!("Failed to get scene tree!");
        }
    }
}

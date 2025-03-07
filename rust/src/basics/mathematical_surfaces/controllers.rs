use godot::classes::{InputEvent, InputEventKey, Label, Node3D};
use godot::prelude::*;

/// Controller for the Mathematical Surface demo
/// Allows users to switch between different mathematical functions using keyboard input
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct MathSurfaceController {
    /// Reference to the title label
    title_label: Option<Gd<Label>>,

    /// Reference to the math surface
    math_surface: Option<Gd<Node3D>>,

    /// Function names for display
    function_names: Vec<String>,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for MathSurfaceController {
    fn ready(&mut self) {
        // Initialize function names
        self.function_names = vec![
            "Wave".into(),
            "Multi Wave".into(),
            "Ripple".into(),
            "Sphere".into(),
            "Torus".into(),
        ];

        // Get references to nodes
        self.title_label = Some(self.base().get_node_as::<Label>("Info/Title"));
        self.math_surface = Some(self.base().get_node_as::<Node3D>("MathSurface"));

        // Update the title
        self.update_title();

        godot_print!("MathSurfaceController ready - use number keys 1-5 to change functions");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed() {
                let mut function_index = -1;

                // Check for number keys 1-5
                let key = key_event.get_keycode();
                let key_str = format!("{:?}", key);

                // 1=Key1, 2=Key2, 3=Key3, 4=Key4, 5=Key5
                if key_str.contains("Key1") {
                    function_index = 0; // Wave
                } else if key_str.contains("Key2") {
                    function_index = 1; // Multi Wave
                } else if key_str.contains("Key3") {
                    function_index = 2; // Ripple
                } else if key_str.contains("Key4") {
                    function_index = 3; // Sphere
                } else if key_str.contains("Key5") {
                    function_index = 4; // Torus
                }

                // Only process valid keys
                if function_index >= 0 {
                    self.change_function(function_index);
                }
            }
        }
    }
}

#[godot_api]
impl MathSurfaceController {
    /// Change the current mathematical function
    fn change_function(&mut self, index: i32) {
        if let Some(ref surface) = self.math_surface {
            // Only update if different
            let current_index = surface.get("function_index").to::<i32>();
            if current_index != index {
                let mut surface_mut = surface.clone();
                let index_variant = Variant::from(index);
                surface_mut.set("function_index", &index_variant);
                self.update_title();
                godot_print!(
                    "Changed function to: {}",
                    self.function_names[index as usize]
                );
            }
        }
    }

    /// Update the title label with current function name
    fn update_title(&self) {
        if let Some(ref label) = self.title_label {
            if let Some(ref surface) = self.math_surface {
                let index = surface.get("function_index").to::<i32>() as usize;
                if index < self.function_names.len() {
                    let text = format!(
                        "Mathematical Surface - {} Function",
                        self.function_names[index]
                    );
                    let mut label_mut = label.clone();
                    label_mut.set_text(&text);
                }
            }
        }
    }
}

/// Controller for the Parametric Surfaces demo
/// Allows adjusting animation speed for both surfaces simultaneously
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct MultiSurfaceController {
    /// Reference to the sphere surface
    sphere_surface: Option<Gd<Node3D>>,

    /// Reference to the torus surface
    torus_surface: Option<Gd<Node3D>>,

    /// Animation speed levels
    speed_levels: Vec<f32>,

    /// Current speed index
    current_speed_index: usize,

    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for MultiSurfaceController {
    fn ready(&mut self) {
        // Initialize speed levels
        self.speed_levels = vec![0.2, 0.5, 1.0, 1.5, 2.0];
        self.current_speed_index = 2; // Start at 1.0 (index 2)

        // Get references to nodes
        self.sphere_surface = Some(self.base().get_node_as::<Node3D>("SphereSurface"));
        self.torus_surface = Some(self.base().get_node_as::<Node3D>("TorusSurface"));

        // Set initial animation speed
        self.set_animation_speed(self.speed_levels[self.current_speed_index]);

        godot_print!("MultiSurfaceController ready - press SPACE to adjust animation speed");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed() {
                let key = key_event.get_keycode();
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
impl MultiSurfaceController {
    /// Set the animation speed for all surfaces
    fn set_animation_speed(&self, speed: f32) {
        if let Some(ref sphere) = self.sphere_surface {
            let mut sphere_mut = sphere.clone();
            let speed_variant = Variant::from(speed);
            sphere_mut.set("animation_speed", &speed_variant);
        }

        if let Some(ref torus) = self.torus_surface {
            let mut torus_mut = torus.clone();
            let speed_variant = Variant::from(speed);
            torus_mut.set("animation_speed", &speed_variant);
        }
    }
}

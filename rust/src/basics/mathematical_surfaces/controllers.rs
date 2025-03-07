use godot::classes::{InputEvent, InputEventKey, Label, Node3D, RandomNumberGenerator};
use godot::prelude::*;

/// Controller for the Mathematical Surface demo
/// Allows changing functions with number keys
#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct MathSurfaceController {
    /// Reference to the title label
    title_label: Option<Gd<Label>>,

    /// Reference to the math surface
    math_surface: Option<Gd<Node3D>>,

    /// Reference to the resolution label (for performance testing)
    resolution_label: Option<Gd<Label>>,

    /// Available resolutions for testing
    resolutions: Vec<i32>,

    /// Current resolution index
    current_resolution_index: usize,

    /// Function names for display
    function_names: Vec<String>,

    /// Current function (can be fractional for transitions)
    current_function: f32,

    /// Target function index
    target_function: i32,

    /// Whether functions cycle automatically
    #[export]
    #[init(val = false)]
    auto_cycle: bool,

    /// Transition mode (cycle or random)
    #[export]
    #[init(val = 0)]
    transition_mode: i32, // 0 = Cycle, 1 = Random

    /// Time between auto-cycling
    #[export]
    #[init(val = 5.0)]
    cycle_time: f32,

    /// Time since last function change
    time_since_change: f32,

    /// Transition speed
    #[export]
    #[init(val = 2.0)]
    transition_speed: f32,

    /// RNG for random mode
    rng: Gd<RandomNumberGenerator>,

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

        // Initialize resolutions for testing
        self.resolutions = vec![20, 40, 60, 100];
        self.current_resolution_index = 1; // Start with 40x40 resolution

        // Initialize RNG
        self.rng = RandomNumberGenerator::new_gd();
        self.rng.randomize();

        // Get references to nodes
        self.title_label = Some(self.base().get_node_as::<Label>("Info/Title"));
        self.math_surface = Some(self.base().get_node_as::<Node3D>("MathSurface"));

        // Try to get the resolution label (might be null if not in performance tester scene)
        if let Some(label_node) = self.base().get_node_or_null("Info/Resolution") {
            self.resolution_label = label_node.try_cast::<Label>().ok();
        }

        // Initialize current function
        if let Some(surface) = &self.math_surface {
            self.current_function = surface.get("function_index").to::<i32>() as f32;
            self.target_function = self.current_function as i32;

            // Set initial resolution from the predefined list
            let resolution = self.resolutions[self.current_resolution_index];
            let mut surface_mut = surface.clone();
            surface_mut.set("resolution", &Variant::from(resolution));

            // Update the resolution label
            self.update_resolution_label();
        }

        // Update the title
        self.update_title();

        godot_print!(
            "MathSurfaceController ready - use number keys 1-5 to change functions, press C to toggle auto cycle, press R to toggle transition mode"
        );
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // Check for keyboard input
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed() {
                // Check for number keys
                let keycode = key_event.get_keycode();

                // Convert keycode to string for comparison
                let key_str = format!("{:?}", keycode);

                if key_str.contains("Key1") {
                    self.change_function(0);
                } else if key_str.contains("Key2") {
                    self.change_function(1);
                } else if key_str.contains("Key3") {
                    self.change_function(2);
                } else if key_str.contains("Key4") {
                    self.change_function(3);
                } else if key_str.contains("Key5") {
                    self.change_function(4);
                } else if key_str.contains("C") {
                    // Toggle auto-cycling
                    self.auto_cycle = !self.auto_cycle;
                    godot_print!("Auto-cycling: {}", self.auto_cycle);
                } else if key_str.contains("R") {
                    // Toggle transition mode between Cycle and Random
                    self.transition_mode = 1 - self.transition_mode;
                    godot_print!(
                        "Transition mode: {}",
                        if self.transition_mode == 0 {
                            "Cycle"
                        } else {
                            "Random"
                        }
                    );
                } else if key_str.contains("B") {
                    // Cycle through resolutions for performance testing
                    self.cycle_resolution();
                }
            }
        }
    }

    fn process(&mut self, delta: f64) {
        let delta_f = delta as f32;

        if self.auto_cycle {
            // Time-based cycling through functions
            self.time_since_change += delta_f;

            if self.time_since_change >= self.cycle_time {
                self.time_since_change = 0.0;

                // Choose next function based on transition mode
                if self.transition_mode == 0 {
                    // Cycle mode - go to next function
                    self.target_function = (self.target_function + 1) % 5;
                } else {
                    // Random mode - pick a random function that's not the current one
                    let current = self.target_function;
                    let mut next;
                    loop {
                        next = (self.rng.randi() % 5) as i32;
                        if next != current {
                            break;
                        }
                    }
                    self.target_function = next;
                }

                godot_print!(
                    "Auto-cycling to function: {}",
                    self.function_names[self.target_function as usize]
                );
            }
        }

        // If we need to update the target function
        if let Some(surface) = &self.math_surface {
            if self.current_function != self.target_function as f32 {
                // Calculate transition progress
                let step = delta_f * self.transition_speed;
                let diff = self.target_function as f32 - self.current_function;

                // Smoothly transition to target
                if diff.abs() <= step {
                    self.current_function = self.target_function as f32;
                } else {
                    self.current_function += step * diff.signum();
                }

                // Calculate transition value for Ch24MathSurface (0.0 to 1.0)
                let whole_part = self.current_function.floor();
                let fractional_part = self.current_function - whole_part;

                // Update the surface with new function index and transition
                let function_index = whole_part as i32;
                let mut surface_mut = surface.clone();
                surface_mut.set("function_index", &Variant::from(function_index));
                surface_mut.set("transition", &Variant::from(fractional_part));

                // Update the title
                self.update_title();
            }
        }
    }
}

#[godot_api]
impl MathSurfaceController {
    fn update_title_with_index(&self, index: i32) {
        if let Some(label) = &self.title_label {
            if index >= 0 && (index as usize) < self.function_names.len() {
                let title = format!(
                    "Mathematical Surface - {} Function",
                    self.function_names[index as usize]
                );
                let mut label_mut = label.clone();
                label_mut.set_text(&title);
            }
        }
    }

    fn update_title(&self) {
        // When transitioning, show blend in title
        if self.current_function.fract() > 0.01 {
            let from_idx = self.current_function.floor() as i32;
            let to_idx = self.target_function;
            if let Some(label) = &self.title_label {
                if from_idx >= 0
                    && (from_idx as usize) < self.function_names.len()
                    && to_idx >= 0
                    && (to_idx as usize) < self.function_names.len()
                {
                    let progress = (self.current_function.fract() * 100.0).round() as i32;
                    let title = format!(
                        "Morphing: {} → {} ({}%)",
                        self.function_names[from_idx as usize],
                        self.function_names[to_idx as usize],
                        progress
                    );
                    let mut label_mut = label.clone();
                    label_mut.set_text(&title);
                }
            }
        } else {
            self.update_title_with_index(self.current_function as i32);
        }
    }

    fn update_resolution_label(&self) {
        if let Some(label) = &self.resolution_label {
            let resolution = self.resolutions[self.current_resolution_index];
            let text = format!("Res: {}x{}", resolution, resolution);
            let mut label_mut = label.clone();
            label_mut.set_text(&text);
        }
    }

    fn cycle_resolution(&mut self) {
        // Cycle to next resolution
        self.current_resolution_index =
            (self.current_resolution_index + 1) % self.resolutions.len();
        let new_resolution = self.resolutions[self.current_resolution_index];

        // Update the math surface resolution
        if let Some(surface) = &self.math_surface {
            let mut surface_mut = surface.clone();
            surface_mut.set("resolution", &Variant::from(new_resolution));

            // Update the label
            self.update_resolution_label();

            godot_print!(
                "Changed resolution to {}x{}",
                new_resolution,
                new_resolution
            );
        }
    }

    fn change_function(&mut self, index: i32) {
        if index != self.target_function {
            godot_print!(
                "Changing to function: {}",
                self.function_names[index as usize]
            );
            self.target_function = index;

            // Reset cycle timer when manually changing function
            self.time_since_change = 0.0;
        }
    }
}

/// Controller for multiple surfaces demo
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
        // Initialize animation speed levels
        self.speed_levels = vec![0.0, 0.25, 0.5, 1.0, 2.0];
        self.current_speed_index = 3; // Default to normal speed (1.0)

        // Get references to nodes
        self.sphere_surface = Some(self.base().get_node_as::<Node3D>("SphereSurface"));
        self.torus_surface = Some(self.base().get_node_as::<Node3D>("TorusSurface"));

        // Set initial animation speed
        self.set_animation_speed(self.speed_levels[self.current_speed_index]);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        // Check for keyboard input
        if let Ok(key_event) = event.try_cast::<InputEventKey>() {
            if key_event.is_pressed() {
                // Check for space key to change animation speed
                let key_str = format!("{:?}", key_event.get_keycode());
                if key_str.contains("Space") {
                    // Cycle through animation speeds
                    self.current_speed_index =
                        (self.current_speed_index + 1) % self.speed_levels.len();
                    let speed = self.speed_levels[self.current_speed_index];
                    self.set_animation_speed(speed);
                    godot_print!("Animation speed: {:.2}", speed);
                }
            }
        }
    }
}

#[godot_api]
impl MultiSurfaceController {
    fn set_animation_speed(&self, speed: f32) {
        if let Some(surface) = &self.sphere_surface {
            let mut surface_mut = surface.clone();
            surface_mut.set("animation_speed", &Variant::from(speed));
        }
        if let Some(surface) = &self.torus_surface {
            let mut surface_mut = surface.clone();
            surface_mut.set("animation_speed", &Variant::from(speed));
        }
    }
}

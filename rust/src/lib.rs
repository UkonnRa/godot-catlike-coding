// Organize the Catlike Coding tutorials into separate modules
mod back_to_home;
mod basics;
mod fps_label;
mod launcher;

// Re-export the Graph classes from the tutorial implementations
pub use basics::building_a_graph::Ch11Graph;
pub use basics::building_a_graph::Ch12Graph;
pub use basics::building_a_graph::Ch13Graph;
pub use basics::building_a_graph::Graph;

// Re-export the MathSurface classes from the Mathematical Surfaces tutorial
pub use basics::mathematical_surfaces::Ch21MathSurface;
pub use basics::mathematical_surfaces::Ch22MathSurface;
pub use basics::mathematical_surfaces::Ch23MathSurface;
pub use basics::mathematical_surfaces::Ch24MathSurface;

// Re-export all controllers
pub use basics::building_a_graph::GraphController;
pub use basics::mathematical_surfaces::MathSurfaceController;
pub use basics::mathematical_surfaces::MultiSurfaceController;
pub use launcher::DemoLauncher;

// Re-export the FPS label
pub use fps_label::FpsLabel;

// Re-export the back to home button
pub use back_to_home::BackToHome;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

// Organize the Catlike Coding tutorials into separate modules
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

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

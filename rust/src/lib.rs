// Organize the Catlike Coding tutorials into separate modules
mod basics;
mod fps_label;

// Re-export the Graph classes from the tutorial implementations
pub use basics::building_a_graph::Ch11Graph;
pub use basics::building_a_graph::Ch12Graph;
pub use basics::building_a_graph::Ch13Graph;
pub use basics::building_a_graph::Graph;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

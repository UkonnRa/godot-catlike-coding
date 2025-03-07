// Mathematical Surfaces Tutorial from Catlike Coding
// https://catlikecoding.com/unity/tutorials/basics/mathematical-surfaces/

pub mod ch2_1_creating_function_library;
pub mod ch2_2_using_delegates;
pub mod ch2_3_surface_creation;
pub mod ch2_4_multiple_dimensions;
pub mod controllers;
pub mod shared;

// Re-export the classes from each chapter
pub use ch2_1_creating_function_library::Ch21MathSurface;
pub use ch2_2_using_delegates::Ch22MathSurface;
pub use ch2_3_surface_creation::Ch23MathSurface;
pub use ch2_4_multiple_dimensions::Ch24MathSurface;

// Re-export controllers
pub use controllers::MathSurfaceController;
pub use controllers::MultiSurfaceController;

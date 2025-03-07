// Building a Graph Tutorial from Catlike Coding
// https://catlikecoding.com/unity/tutorials/basics/building-a-graph/

pub mod ch1_1_creating_a_line_of_cubes;
pub mod ch1_2_creating_a_grid_of_cubes;
pub mod ch1_3_creating_a_graph;
pub mod ch1_4_animating_the_graph;
pub mod shared;

// Re-export the classes from each chapter
pub use ch1_1_creating_a_line_of_cubes::Ch11Graph;
pub use ch1_2_creating_a_grid_of_cubes::Ch12Graph;
pub use ch1_3_creating_a_graph::Ch13Graph;
pub use ch1_4_animating_the_graph::Graph;

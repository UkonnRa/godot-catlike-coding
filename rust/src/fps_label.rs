use godot::classes::Label;
use godot::prelude::*;

/// FPS label to monitor performance
/// Displays frames per second (FPS) and milliseconds per frame (MS)
#[derive(GodotClass)]
#[class(init, base=Label)]
pub struct FpsLabel {
    /// Frames counted so far
    frames: u32,

    /// Time elapsed since last update
    elapsed_time: f64,

    /// Update the display every 0.5 seconds
    #[export]
    #[init(val = 0.5)]
    update_interval: f64,

    #[base]
    base: Base<Label>,
}

#[godot_api]
impl FpsLabel {
    /// Called when the node is ready
    #[func]
    fn _ready(&mut self) {
        // Set initial text
        self.base_mut().set_text("FPS: --- | MS: ---");
    }

    /// Process called every frame
    #[func]
    fn _process(&mut self, delta: f64) {
        // Count frames
        self.frames += 1;
        self.elapsed_time += delta;

        // Update the label text every update_interval seconds
        if self.elapsed_time >= self.update_interval {
            // Calculate FPS and MS
            let fps = self.frames as f64 / self.elapsed_time;
            let ms = self.elapsed_time * 1000.0 / self.frames as f64;

            // Update the label
            let text = format!("FPS: {:.1} | MS: {:.1}", fps, ms);
            self.base_mut().set_text(&text);

            // Reset counters
            self.frames = 0;
            self.elapsed_time = 0.0;
        }
    }
}

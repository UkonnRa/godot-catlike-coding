use godot::classes::{Engine, ILabel, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Label)]
struct FPSLabel {
    #[base]
    base: Base<Label>,
}

#[godot_api]
impl ILabel for FPSLabel {
    fn process(&mut self, delta: f64) {
        let text = format!(
            "{} FPS - {:.2} ms",
            Engine::singleton().get_frames_per_second(),
            delta * 1000.0
        );
        self.base_mut().set_text(&text);
    }
}

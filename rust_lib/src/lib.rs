use egui::{CtxRef, Window};
use gdnative::prelude::*;
use godot_egui::GodotEgui;

#[derive(NativeClass)]
#[inherit(Spatial)]
struct EguiTest {
    egui_test: egui_demo_lib::ColorTest,
}

#[gdnative::methods]
impl EguiTest {
    fn new(_owner: &Spatial) -> Self {
        Self {
            egui_test: egui_demo_lib::ColorTest::default(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &Spatial) {
        godot_print!("Test node ready")
    }

    #[export]
    fn _process(&mut self, owner: &Spatial, _delta: f64) {
        let gui = unsafe { owner.get_node_as_instance::<GodotEgui>("egui").unwrap() };
        gui.map_mut(|gui, instance| {
            gui.update_ctx(instance, |ctx| self.draw(ctx));
        })
        .expect("egui error");
    }

    fn draw(&mut self, ctx: &mut CtxRef) {
        Window::new("Color Test").scroll(true).show(ctx, |ui| {
            self.egui_test.ui(ui, &mut None);
        });
        Window::new("Settings").scroll(true).show(ctx, |ui| {
            ctx.settings_ui(ui);
        });
    }
}

fn init(handle: InitHandle) {
    godot_egui::register_classes(handle);
    handle.add_class::<EguiTest>();
}

godot_init!(init);

use eframe::{egui::CentralPanel, epi::App, epi::NativeOptions, run_native};

struct Headlines;

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines;
    let win_options = NativeOptions::default();
    run_native(Box::new(app), win_options)
}

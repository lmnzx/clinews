use eframe::{
    egui::{CentralPanel, ScrollArea, Vec2},
    epi::App,
    epi::NativeOptions,
    run_native,
};

struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    fn new() -> Headlines {
        let iter = (0..20).map(|i| NewsCardData {
            title: format!("Headline {}", i),
            description: format!("This is the description of headline {}", i),
            url: format!("https://www.rust-lang.org/"),
        });
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx);
    }

    fn configure_fonts(&self, ctx: &eframe::egui::CtxRef) {}

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for article in &self.articles {
                    ui.label(article.title.clone());
                    ui.label(article.description.clone());
                    ui.label(article.url.clone());
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn main() {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_options)
}

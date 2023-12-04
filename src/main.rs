mod headlines;

use eframe::{epi::App, run_native, egui::{CentralPanel, Ui, ScrollArea, Vec2, Separator, TopBottomPanel, CtxRef, Label, Hyperlink, TextStyle}, NativeOptions};
use headlines::Headlines;

const PADDING: f32 = 5.0;

impl App for Headlines {
    fn setup(
            &mut self,
            ctx: &eframe::egui::CtxRef,
            _frame: &mut eframe::epi::Frame<'_>,
            _storage: Option<&dyn eframe::epi::Storage>,
        ) {
        self.configure_font(ctx);
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        self.render_top_panel(ctx);
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            render_header(ui);
            ScrollArea::auto_sized().show(ui, |ui: &mut Ui| {
                self.render_news_cards(ui);
            });
            render_footer(ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui: &mut Ui| {
        ui.vertical_centered(|ui: &mut Ui| {
            ui.add_space(10.);
            // add api source website
            ui.add(Label::new("API source: newsapi.org").monospace());
            // add link to egui framework
            ui.add(Hyperlink::new("https://google.com").text("Made with egui").text_style(TextStyle::Monospace));
            // put github link
            ui.add(Hyperlink::new("https://google.com").text("Github").text_style(TextStyle::Monospace));
            ui.add_space(10.);
        })
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui: &mut Ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn main() {
    let app: Headlines = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_option);
}

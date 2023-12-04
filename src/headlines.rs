use std::borrow::Cow;

use eframe::egui::{Color32, CtxRef, FontDefinitions, TextStyle, FontFamily, Ui, Label, Layout, Hyperlink, Separator, TopBottomPanel};

const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct Headlines {
    articles: Vec<NewsCardData>
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String
}

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a: i32| NewsCardData {
            title: format!("title{}", a),
            desc: format!("desc{}", a),
            url: format!("https://example.com/{}", a)
        });
        Headlines {
            articles: Vec::from_iter(iter)
        }
    }

    pub fn configure_font(&self, ctx: &CtxRef) {
        // create font def object
        let mut font_def = FontDefinitions::default();
        // load font
        font_def.font_data.insert("MesloLGS".to_string(), Cow::Borrowed(include_bytes!("../MesloLGS_NF_Regular.ttf")));
        // set fint size and styles
        font_def.family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def.family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 20.));
        // load the font
        font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "MesloLGS".to_string());
        ctx.set_fonts(font_def);
    }

    pub fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶️ {}", a.title);
            ui.colored_label(WHITE, title);
            // render desc
            ui.add_space(PADDING);
            let desc = Label::new(&a.desc).text_style(TextStyle::Button);
            ui.add(desc);

            // render hyperlinks
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(), |ui: &mut Ui| {
                ui.add(Hyperlink::new(&a.url).text("read more ⤴️"));
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&self, ctx: &CtxRef) {
        // define TopBottomPanel
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            ui.add_space(10.);
            eframe::egui::menu::bar(ui, |ui: &mut Ui| {
                // Logo
                ui.with_layout(Layout::left_to_right(), |ui: &mut Ui| {
                    ui.add(Label::new("Test").text_style(eframe::egui::TextStyle::Heading));
                });
                // Controls
                ui.with_layout(Layout::right_to_left(), |ui: &mut Ui| {
                    let close_btn = ui.add(eframe::egui::Button::new("❌").text_style(TextStyle::Body));
                    let refresh_btn = ui.add(eframe::egui::Button::new("🔁").text_style(TextStyle::Body));
                    let theme_btn = ui.add(eframe::egui::Button::new("🌙").text_style(TextStyle::Body));
                })
            });
            ui.add_space(10.);
        });
        // add menu bar

        // two layout widgets

        // render logo on the left

        // add control buttons on the right

        // padding
    }
}
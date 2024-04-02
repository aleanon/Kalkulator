use egui::style::{WidgetVisuals, Widgets};
use egui::{Color32, FontId, Rounding, Stroke, Style, Visuals};

use crate::numpad::Numpad;
use crate::render::{self, BACKGROUND_COLOR};

use super::standard::Standard;

pub const ROUNDING: f32 = 5.;

pub struct Kalkulator {
    pub kind: Standard,
    pub numpad: Numpad,
}

impl Kalkulator {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        //cc.egui_ctx.set_os(egui::os::OperatingSystem::Windows);

        cc.egui_ctx.set_style(Self::custom_style());

        Self {
            kind: Standard::new(),
            numpad: Numpad::new(),
        }
    }

    fn custom_style() -> Style {
        Style {
            override_text_style: Some(egui::TextStyle::Body),
            override_font_id: Some(FontId::monospace(15.)),
            visuals: Self::custom_visuals(),

            ..Style::default()
        }
    }

    fn custom_visuals() -> Visuals {
        Visuals {
            button_frame: false,
            dark_mode: true,
            panel_fill: BACKGROUND_COLOR,
            window_fill: BACKGROUND_COLOR,
            widgets: Self::custom_widgets(),

            ..Visuals::default()
        }
    }

    fn custom_widgets() -> Widgets {
        Widgets {
            noninteractive: WidgetVisuals {
                weak_bg_fill: Color32::TRANSPARENT,
                bg_fill: BACKGROUND_COLOR,
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // separators, indentation lines
                fg_stroke: Stroke::new(1.0, Color32::from_gray(160)), // normal text color
                rounding: Rounding::same(ROUNDING),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                weak_bg_fill: Color32::TRANSPARENT,
                bg_fill: BACKGROUND_COLOR, // button background
                bg_stroke: Default::default(),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)), // button text
                rounding: Rounding::same(ROUNDING),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                weak_bg_fill: BACKGROUND_COLOR.linear_multiply(0.25),
                bg_fill: BACKGROUND_COLOR.linear_multiply(0.25),
                bg_stroke: Stroke::new(0.0, BACKGROUND_COLOR.linear_multiply(1.)), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(ROUNDING),
                expansion: 0.0,
            },
            active: WidgetVisuals {
                weak_bg_fill: Color32::TRANSPARENT,
                bg_fill: Color32::from_gray(55),
                bg_stroke: Stroke::new(1.0, Color32::WHITE),
                fg_stroke: Stroke::new(2.0, Color32::WHITE),
                rounding: Rounding::same(ROUNDING),
                expansion: 0.0,
            },
            open: WidgetVisuals {
                weak_bg_fill: Color32::from_gray(27),
                bg_fill: Color32::from_gray(27),
                bg_stroke: Stroke::new(1.0, Color32::from_gray(60)),
                fg_stroke: Stroke::new(1.0, Color32::from_gray(210)),
                rounding: Rounding::same(ROUNDING),
                expansion: 0.0,
            },
        }
    }
}

impl eframe::App for Kalkulator {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        render::top_panel(ctx, frame);

        render::central_panel(self, ctx, frame)
    }
}

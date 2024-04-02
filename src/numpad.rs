use eframe::epaint::Shadow;
use egui::{
    style::{WidgetVisuals, Widgets}, Color32, Context, Frame, Margin, Rounding, Stroke, Style, Visuals
};

use crate::{
    kalkulator::ROUNDING,
    render::{BACKGROUND_COLOR, TEXT_COLOR},
};

pub struct Numpad {
    pub dark_button_context: Context,

    pub light_button_context: Context,

    pub frame: Frame,

    pub dark_button_style: Style,

    pub light_button_style: Style,

    pub dark_visuals: Visuals,

    pub light_visuals: Visuals,
}

impl Numpad {
    pub fn new() -> Self {
        let numpad = Self {
            dark_button_context: Self::context(),
            light_button_context: Self::context(),
            frame: Self::frame(),
            dark_button_style: Self::dark_style(),
            light_button_style: Self::light_style(),
            dark_visuals: Self::dark_visuals(),
            light_visuals: Self::light_visuals(),
        };

        numpad.dark_button_context.set_visuals(Self::dark_visuals());
        numpad
            .light_button_context
            .set_visuals(Self::light_visuals());
        numpad
    }

    fn context() -> Context {
        Context::default()
    }

    fn frame() -> Frame {
        Frame {
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            rounding: Rounding::same(ROUNDING),
            shadow: Shadow::NONE,
            fill: BACKGROUND_COLOR,
            stroke: Stroke::default(),
        }
    }

    pub fn dark_visuals() -> Visuals {
        Visuals {
            widgets: Widgets {
                inactive: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: BACKGROUND_COLOR.linear_multiply(0.375),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(0.0, TEXT_COLOR),
                    expansion: 0.0,
                    rounding: Rounding::same(ROUNDING),
                },
                hovered: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: BACKGROUND_COLOR.linear_multiply(1.),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(0.0, TEXT_COLOR),
                    expansion: 0.0,
                    rounding: Rounding::same(ROUNDING),
                },
                ..Widgets::dark()
            },
            ..Default::default()
        }
    }

    fn light_visuals() -> Visuals {
        Visuals {
            widgets: Widgets {
                inactive: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: BACKGROUND_COLOR.linear_multiply(1.),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(0.0, TEXT_COLOR),
                    expansion: 0.0,
                    rounding: Rounding::same(ROUNDING),
                },
                hovered: WidgetVisuals {
                    weak_bg_fill: Color32::TRANSPARENT,
                    bg_fill: BACKGROUND_COLOR.linear_multiply(0.375),
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(0.0, TEXT_COLOR),
                    expansion: 0.0,
                    rounding: Rounding::same(ROUNDING),
                },
                ..Widgets::dark()
            },
            ..Default::default()
        }
    }

    pub fn dark_style() -> Style {
        Style {
            visuals: Self::dark_visuals(),
            ..Default::default()
        }
    }

    fn light_style() -> Style {
        Style {
            visuals: Self::light_visuals(),
            ..Default::default()
        }
    }
}

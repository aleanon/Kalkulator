use egui::panel::TopBottomPanel;
use egui::{
    Align, Button, CentralPanel, Color32, Context, FontId, Frame, Grid, Key,
    Label, Layout, RichText, SelectableLabel, Vec2, Widget,
};

use crate::kalkulator::Kalkulator;

pub const BACKGROUND_COLOR: Color32 = Color32::from_rgba_premultiplied(23, 23, 23, 0);
pub const TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const NUM_PAD_TEXT_SIZE: f32 = 15.0;
const DARK_BUTTON_MUL: f32 = 0.600;
const LIGHT_BUTTON_MUL: f32 = 1.1;

pub fn top_panel(ctx: &Context, _frame: &mut eframe::Frame) {
    TopBottomPanel::top("Top Panel")
        .show_separator_line(false)
        .exact_height(30.)
        .show(ctx, |ui| {
            ui.add_space(9.);
            egui::menu::bar(ui, |ui| {
                //ui.add_space(6.);
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), 30.),
                    Layout::left_to_right(Align::LEFT),
                    |ui| {
                        ui.add_sized(
                            [ui.available_height(), ui.available_height()],
                            SelectableLabel::new(
                                false,
                                RichText::new("").font(FontId::proportional(15.)),
                            ),
                        );
                        //ui.add_space(4.);
                        ui.add_sized(
                            [ui.available_height(), ui.available_height()],
                            Label::new(
                                RichText::new("Standard")
                                    .color(Color32::from_rgb(255, 255, 255))
                                    .size(20.),
                            ),
                        );
                        //ui.add_space(8.);
                        ui.add_sized(
                            [ui.available_height(), ui.available_height()],
                            SelectableLabel::new(
                                false,
                                RichText::new("🗖").font(FontId::proportional(15.)),
                            ),
                        );
                    },
                );
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    Layout::right_to_left(Align::RIGHT),
                    |ui| {
                        ui.add_sized(
                            [ui.available_height(), ui.available_height()],
                            SelectableLabel::new(
                                false,
                                RichText::new("⟲")
                                    .color(Color32::from_rgb(255, 255, 255))
                                    .size(20.),
                            ),
                        )
                    },
                )
            });
            ui.add_space(3.)
        });
}

pub fn central_panel(kalkulator: &mut Kalkulator, ctx: &Context, _frame: &mut eframe::Frame) {
    CentralPanel::default()
        .frame(Frame::default().fill(BACKGROUND_COLOR))
        .show(ctx, |ui| {
            ui.allocate_ui_at_rect(ui.max_rect().shrink(5.), |ui| {
                //Top screen
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), 20.),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.add_space(20.);
                        ui.label(
                            RichText::new(kalkulator.kind.read_cache().concat())
                                .color(Color32::from_rgb(170, 170, 170))
                                .size(13.)
                                .monospace(),
                        )
                    },
                );

                ui.add_space(6.);

                //Main screen
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.min_size()[0], ui.available_height() / 7.5),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.add_space(10.);
                        ui.label(
                            RichText::new(kalkulator.kind.read_buffer())
                                .color(Color32::from_rgb(255, 255, 255))
                                .font(egui::FontId::new(
                                    match ui.available_width().round() as i32 {
                                        0..=320 => 35.0,
                                        321..=400 => 40.0,
                                        401..=480 => 45.0,
                                        _ => 50.0,
                                    },
                                    egui::FontFamily::Proportional,
                                )),
                        )
                    },
                );

                ui.add_space(10.);

                Grid::new("MC Buttons")
                    .min_row_height(ui.available_height() / 12.5)
                    .max_col_width(ui.available_width())
                    .spacing(Vec2::new(5.0, 5.0))
                    .show(ui, |grid| {
                        let width = (grid.available_width() - 25.) / 6.;
                        let height = grid.available_height();

                        grid.allocate_ui(Vec2::new(width, height), |ui| {
                            ui.add_enabled_ui(false, |ui| {
                                Button::new(RichText::new("MC").font(FontId::proportional(12.)))
                                    .ui(ui)
                            })
                        });
                        // grid.add_sized(
                        //     [width, height],
                        //     SelectableLabel::new(
                        //         false,
                        //         RichText::new("MC").font(FontId::proportional(12.)),
                        //     ),
                        // );
                        grid.add_sized(
                            [width, height],
                            SelectableLabel::new(
                                false,
                                RichText::new("MR").font(FontId::proportional(12.)),
                            ),
                        );
                        grid.add_sized(
                            [width, height],
                            SelectableLabel::new(
                                false,
                                RichText::new("M+").font(FontId::proportional(12.)),
                            ),
                        );
                        grid.add_sized(
                            [width, height],
                            SelectableLabel::new(
                                false,
                                RichText::new("M-").font(FontId::proportional(12.)),
                            ),
                        );
                        grid.add_sized(
                            [width, height],
                            SelectableLabel::new(
                                false,
                                RichText::new("MS").font(FontId::proportional(12.)),
                            ),
                        );
                        grid.add_sized(
                            [width, height],
                            SelectableLabel::new(
                                false,
                                RichText::new("MC").font(FontId::proportional(12.)),
                            ),
                        );
                    });
                ui.add_space(2.0);

                Grid::new("bleh")
                    .min_row_height((ui.available_height() - 10.) / 6.)
                    .max_col_width(ui.available_width() - 9.)
                    .spacing(Vec2::new(3.0, 2.0))
                    .show(ui, |grid| {
                        let width = grid.available_width() / 4.;
                        let height = grid.available_height();

                        // grid.allocate_ui(Vec2::new(grid.available_width(), height), |ui| {
                        //     ui.centered_and_justified(|ui| {
                        //         Frame::none()
                        //             .fill(BACKGROUND_COLOR.linear_multiply(0.375))
                        //             .stroke(Stroke::new(0.0, BACKGROUND_COLOR.linear_multiply(1.)))
                        //             .rounding(Rounding::same(ROUNDING))
                        //             .show(ui, |ui| {
                        //                 ui.button(RichText::new("%").size(NUM_PAD_TEXT_SIZE))
                        //             })
                        //     })
                        // });

                        grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("%").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("CE").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        let clear = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("C").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if clear.clicked() {
                            kalkulator.kind.clear()
                        }
                        let backspace = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("⌫").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if backspace.clicked() || grid.input(|i| i.key_pressed(Key::Backspace)) {
                            kalkulator.kind.backspace()
                        }
                        grid.end_row();
                        grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("1/x").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        let square = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("x^2").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if square.clicked() {
                            kalkulator.kind.square()
                        }
                        grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("√").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        let divide = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("÷").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if divide.clicked() {
                            kalkulator.kind.divide()
                        }
                        grid.end_row();
                        let seven = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("7").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if seven.clicked() || grid.input(|i| i.key_pressed(Key::Num7)) {
                            kalkulator.kind.input_char('7')
                        }
                        let eight = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("8").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if eight.clicked() || grid.input(|i| i.key_pressed(Key::Num8)) {
                            kalkulator.kind.input_char('8')
                        }

                        let nine = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("9").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );

                        if nine.clicked() || grid.input(|i| i.key_pressed(Key::Num9)) {
                            kalkulator.kind.input_char('9')
                        }

                        let multiply = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("x").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if multiply.clicked() {
                            kalkulator.kind.multiply()
                        }
                        grid.end_row();
                        let four = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("4").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if four.clicked() || grid.input(|i| i.key_pressed(Key::Num4)) {
                            kalkulator.kind.input_char('4')
                        }
                        let five = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("5").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if five.clicked() || grid.input(|i| i.key_pressed(Key::Num5)) {
                            kalkulator.kind.input_char('5')
                        }
                        let six = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("6").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if six.clicked() || grid.input(|i| i.key_pressed(Key::Num6)) {
                            kalkulator.kind.input_char('6')
                        }
                        let minus = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("-").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if minus.clicked() || grid.input(|i| i.key_pressed(Key::Minus)) {
                            kalkulator.kind.subtract()
                        }

                        grid.end_row();
                        let one = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("1").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if one.clicked() || grid.input(|i| i.key_pressed(Key::Num1)) {
                            kalkulator.kind.input_char('1')
                        }
                        let two = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("2").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if two.clicked() || grid.input(|i| i.key_pressed(Key::Num2)) {
                            kalkulator.kind.input_char('2')
                        }
                        let three = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("3").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(LIGHT_BUTTON_MUL)),
                        );
                        if three.clicked() || grid.input(|i| i.key_pressed(Key::Num3)) {
                            kalkulator.kind.input_char('3')
                        }
                        let add = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("+").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if add.clicked() {
                            kalkulator.kind.add()
                        }

                        grid.end_row();

                        grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("∓").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(1.)),
                        );
                        let zero = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("0").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(1.)),
                        );
                        if zero.clicked() || grid.input(|i| i.key_pressed(Key::Num0)) {
                            kalkulator.kind.input_char('0')
                        }

                        let comma = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new(",").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(1.)),
                        );
                        if comma.clicked() {
                            kalkulator.kind.input_char(',')
                        }
                        let equal = grid.add_sized(
                            [width, height],
                            Button::new(RichText::new("=").size(NUM_PAD_TEXT_SIZE))
                                .fill(BACKGROUND_COLOR.linear_multiply(DARK_BUTTON_MUL)),
                        );
                        if equal.clicked() || grid.input(|i| i.key_pressed(Key::Enter)) {
                            kalkulator.kind.equal()
                        }
                    });

                //let strip_width = ui.available_width();

                // StripBuilder::new(ui)
                //     .cell_layout(Layout::right_to_left(Align::TOP))
                //     .size(Size::relative(0.25).at_most(strip_width))
                //     .sizes(Size::remainder(), 4)
                //     .horizontal(|mut strip| {
                //         strip.cell(|mut cell| {
                //             cell.button(RichText::new("MC"));
                //         });
                //         strip.cell(|mut cell| {
                //             cell.button(RichText::new("MC"));
                //         });
                //         strip.cell(|mut cell| {
                //             cell.button(RichText::new("MC"));
                //         });
                //         strip.cell(|mut cell| {
                //             cell.button(RichText::new("MC"));
                //         });
                //     });

                // let table_width = ui.available_width();
                // let row_height = (ui.available_height() / 6.) - 10.;

                // TableBuilder::new(ui)
                //     .columns(Column::remainder(), 4)
                //     .body(|body| {
                //         body.rows(row_height, 6_0, |row_index, mut row| {
                //             row.col(|mut cell| {
                //                 cell.button(RichText::new("MC"));
                //             });
                //             row.col(|mut cell| {
                //                 cell.button(RichText::new("MC"));
                //             });
                //             row.col(|mut cell| {
                //                 cell.button(RichText::new("MC"));
                //             });
                //             row.col(|mut cell| {
                //                 cell.button(RichText::new("MC"));
                //             });
                //         })
                //     });

                // ui.allocate_ui_with_layout(
                //     Vec2::new(ui.available_width(), ui.available_height() / 10.),
                //     Layout::left_to_right(Align::BOTTOM),
                //     |ui| {
                //         let button_width = ui.available_width() / 6.;
                //         let height = ui.available_height();
                //         //ui.add_space(3.);
                //         ui.add_sized([button_width, height], SelectableLabel::new(false, "MC"));
                //         ui.add_sized([button_width, height], SelectableLabel::new(false, "MR"));
                //         ui.add_sized([button_width, height], SelectableLabel::new(false, "M+"));
                //         ui.add_sized([button_width, height], SelectableLabel::new(false, "M-"));
                //         ui.add_sized([button_width, height], SelectableLabel::new(false, "MS"));
                //         ui.add_sized([button_width, height], SelectableLabel::new(false, "M⏷"));
                //         //ui.add_space(3.);
                //     },
                // )
            });
        });
}

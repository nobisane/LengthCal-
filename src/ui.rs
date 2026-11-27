//! The screen: a scrolling list of unit boxes with a number pad underneath.

use crate::convert::{Calculator, UNITS};
use eframe::egui::{self, Align2, Color32, FontId, Rounding, Sense, Stroke, Vec2};

const ACCENT: Color32 = Color32::from_rgb(242, 140, 40);
const BOX_BG: Color32 = Color32::from_rgb(38, 41, 48);
const MUTED: Color32 = Color32::from_rgb(165, 170, 182);

#[derive(Default)]
pub struct LengthCal {
    calc: Calculator,
}

impl eframe::App for LengthCal {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        egui::TopBottomPanel::top("title").show(ctx, |ui| {
            // Room for the status bar.
            ui.add_space(32.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("lengthcal")
                        .size(24.0)
                        .strong()
                        .color(ACCENT),
                );
            });
            ui.add_space(6.0);
        });

        egui::TopBottomPanel::bottom("keypad").show(ctx, |ui| {
            ui.add_space(6.0);
            self.keypad(ui);
            // Room for the navigation bar.
            ui.add_space(24.0);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for i in 0..UNITS.len() {
                        self.unit_box(ui, i);
                    }
                });
        });
    }
}

impl LengthCal {
    fn unit_box(&mut self, ui: &mut egui::Ui, i: usize) {
        let (name, symbol, _) = UNITS[i];
        let size = Vec2::new(ui.available_width(), 58.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        if response.clicked() {
            self.calc.select(i);
        }

        let active = self.calc.active == i;
        let border = if active {
            ACCENT
        } else {
            Color32::from_gray(70)
        };
        let painter = ui.painter();
        painter.rect(
            rect,
            Rounding::same(10.0),
            BOX_BG,
            Stroke::new(if active { 2.0 } else { 1.0 }, border),
        );

        let inner = rect.shrink2(Vec2::new(12.0, 6.0));
        painter.text(
            inner.left_top(),
            Align2::LEFT_TOP,
            format!("{name} ({symbol})"),
            FontId::proportional(13.0),
            if active { ACCENT } else { MUTED },
        );

        let value = self.calc.text(i);
        let (shown, color) = if value.is_empty() {
            ("0".to_string(), Color32::from_gray(100))
        } else {
            (value, Color32::WHITE)
        };
        painter.text(
            inner.right_bottom(),
            Align2::RIGHT_BOTTOM,
            shown,
            FontId::proportional(24.0),
            color,
        );
    }

    fn keypad(&mut self, ui: &mut egui::Ui) {
        const ROWS: [[&str; 3]; 4] = [
            ["7", "8", "9"],
            ["4", "5", "6"],
            ["1", "2", "3"],
            [".", "0", "DEL"],
        ];
        let gap = ui.spacing().item_spacing.x;
        let width = (ui.available_width() - 2.0 * gap) / 3.0;

        for row in ROWS {
            ui.horizontal(|ui| {
                for key in row {
                    if key_button(ui, key, width) {
                        match key {
                            "DEL" => self.calc.backspace(),
                            _ => key.chars().for_each(|c| self.calc.press(c)),
                        }
                    }
                }
            });
        }
        let full = ui.available_width();
        if key_button(ui, "CLEAR", full) {
            self.calc.clear();
        }
    }
}

fn key_button(ui: &mut egui::Ui, label: &str, width: f32) -> bool {
    let text = egui::RichText::new(label).size(22.0);
    ui.add_sized([width, 46.0], egui::Button::new(text))
        .clicked()
}

use super::GsxmanApp;
use crate::core::constants;
use egui::{Id, Margin};

mod map_panel;
mod menu_bar_panel;
pub mod plugins;
mod table_panel;

impl eframe::App for GsxmanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let content_width = if let Some(rect) = ctx.input(|i| i.viewport().inner_rect) {
            rect.width()
        } else {
            constants::WINDOW_SIZE.x
        };

        let rimless = egui::Frame {
            fill: ctx.style().visuals.panel_fill,
            inner_margin: Margin::symmetric(5.0, 5.0),
            ..Default::default()
        };

        egui::TopBottomPanel::top(Id::new("top_panel")).show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    menu_bar_panel::update_menu_bar_panel(self, ui);
                });
            });
        });

        egui::SidePanel::left(Id::new("map_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width((content_width / 2.0) - 5.0)
            .show(ctx, |ui| {
                map_panel::update_map_panel(self, ui);
            });

        egui::SidePanel::right(Id::new("configlist_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width((content_width / 2.0) - 5.0)
            .show(ctx, |ui| {
                table_panel::update_table_panel(self, ui);
            });
    }
}

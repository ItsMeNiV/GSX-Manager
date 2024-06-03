use egui::{Id, Margin};

use crate::core::{constants, GsxSection, ProfileFile};

use super::GsxmanApp;

mod map_panel;
mod menu_bar_panel;
pub mod plugins;
mod table_panel;

pub enum UIState {
    Overview,
    Details,
    SectionDetails,
}

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

        let mut left_panel_width = 0.0;

        egui::SidePanel::left(Id::new("map_panel"))
            .frame(rimless)
            .resizable(true)
            .default_width((content_width / 2.5) - 5.0)
            .min_width(50.0)
            .show(ctx, |ui| {
                map_panel::update_map_panel(self, ui);
                left_panel_width = ui.available_width();
            });

        egui::SidePanel::right(Id::new("profilelist_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width(content_width - left_panel_width)
            .show(ctx, |ui| {
                table_panel::update_table_panel(self, ui);
            });
    }
}

fn filter_profiles(filter_text: &str, profile: &ProfileFile) -> bool {
    let mut should_display;

    if filter_text.is_empty() {
        return true;
    }

    let filter_text_lowercase = filter_text.to_lowercase();
    let filter_str = filter_text_lowercase.as_str();

    should_display = profile.airport.icao.to_lowercase().contains(filter_str)
        || profile.airport.name.to_lowercase().contains(filter_str);

    if !should_display {
        if let Some(profile_data) = profile.profile_data.clone() {
            should_display = profile_data.creator.to_lowercase().contains(filter_str);
        }
    }

    should_display
}

fn filter_profile_details(filter_text: &str, section: &GsxSection) -> bool {
    if filter_text.is_empty() {
        return true;
    }

    let filter_text_lowercase = filter_text.to_lowercase();
    let filter_str = filter_text_lowercase.as_str();

    section.name.to_lowercase().contains(filter_str)
}
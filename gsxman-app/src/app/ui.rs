use egui::{Id, Ui};
use egui_extras::{Column, TableBuilder};
use walkers::{
    extras::{Place, Places, Style},
    Map, Position,
};

use crate::core::constants;

use super::GsxmanApp;

impl GsxmanApp {
    fn update_map_panel(&mut self, ui: &mut Ui) {
        let places = Places::new(
            self.installed_gsx_profiles
                .iter()
                .map(|profile| Place {
                    label: profile.airport.icao.to_owned(),
                    position: Position::from_lat_lon(
                        profile.airport.location.latitude(),
                        profile.airport.location.longitude(),
                    ),
                    symbol: '✈',
                    style: Style::default(),
                })
                .collect(),
        );

        ui.add(
            Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lat_lon(52.0, 0.0),
            )
            .zoom_gesture(true)
            .with_plugin(places),
        );
    }

    fn update_table_panel(&mut self, ui: &mut Ui) {
        ui.heading("Installed GSX Profiles");
        ui.separator();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().clip(false))
            .column(Column::remainder().clip(false))
            .column(Column::remainder().clip(false))
            .column(Column::remainder().clip(false));
            
            table = table.sense(egui::Sense::click());

            table.header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("ICAO");
                });
                header.col(|ui| {
                    ui.heading("Airport Name");
                });
                header.col(|ui| {
                    ui.heading("File Location");
                });
            })
            .body(|mut body| {
                for profile in &self.installed_gsx_profiles {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.label(profile.airport.icao.to_string());
                        });
                        row.col(|ui| {
                            ui.label(profile.airport.name.to_string());
                        });
                        row.col(|ui| {
                            ui.label(profile.file_location.as_os_str().to_str().unwrap());
                        });
                    });
                }
            });
    }
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
            ..Default::default()
        };

        egui::SidePanel::left(Id::new("map_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width(content_width / 2.0)
            .show(ctx, |ui| {
                self.update_map_panel(ui);
            });

        egui::SidePanel::right(Id::new("configlist_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width(content_width / 2.0)
            .show(ctx, |ui| {
                self.update_table_panel(ui);
            });
    }
}

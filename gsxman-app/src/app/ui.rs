use super::GsxmanApp;
use crate::core::{constants, filehandler, Airport};
use egui::{menu, Color32, Id, Ui};
use egui_extras::{Column, TableBuilder};
use tracing::error;
use walkers::{
    extras::{Place, Places, Style},
    Map, Plugin, Position,
};

#[derive(Default)]
pub struct ClickWatcher {
    pub clicked_airport: Option<Airport>,
}

impl Plugin for &mut ClickWatcher {
    fn run(
        &mut self,
        response: &egui::Response,
        painter: egui::Painter,
        projector: &walkers::Projector,
    ) {
        let click_position =
            if !response.changed() && response.clicked_by(egui::PointerButton::Primary) {
                response.interact_pointer_pos()
            } else {
                None
            };

        if let Some(position) = click_position {
            error!("{:?}", position);
        }
    }
}

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
                    style: Style {
                        label_background: if let Some(selected_profile) = &self.selected_profile {
                            if selected_profile.airport.icao == profile.airport.icao {
                                Color32::BLUE.gamma_multiply(0.8)
                            } else {
                                Color32::BLACK.gamma_multiply(0.8)
                            }
                        } else {
                            Color32::BLACK.gamma_multiply(0.8)
                        },
                        symbol_background: if let Some(selected_profile) = &self.selected_profile {
                            if selected_profile.airport.icao == profile.airport.icao {
                                Color32::BLUE.gamma_multiply(0.8)
                            } else {
                                Color32::WHITE.gamma_multiply(0.8)
                            }
                        } else {
                            Color32::WHITE.gamma_multiply(0.8)
                        },
                        ..Default::default()
                    },
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
            .with_plugin(places)
            .with_plugin(&mut self.click_watcher),
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

        table
            .header(20.0, |mut header| {
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
                        if let Some(selected_profile) = &self.selected_profile {
                            row.set_selected(selected_profile.airport.icao == profile.airport.icao);
                        }

                        row.col(|ui| {
                            ui.label(profile.airport.icao.to_string());
                        });
                        row.col(|ui| {
                            ui.label(profile.airport.name.to_string());
                        });
                        row.col(|ui| {
                            ui.label(profile.file_location.as_os_str().to_str().unwrap());
                        });

                        if row.response().clicked() {
                            if let Some(selected_profile) = &self.selected_profile {
                                if selected_profile.airport.icao == profile.airport.icao {
                                    self.selected_profile = None
                                } else {
                                    self.selected_profile = Some(profile.clone());
                                }
                            } else {
                                self.selected_profile = Some(profile.clone());
                            }
                        }
                    });
                }
            });
    }

    fn update_menu_bar_panel(&mut self, ui: &mut Ui) {
        menu::bar(ui, |ui| {
            if ui.button("Import Profile").clicked() {
                filehandler::import_config_file_dialog();
                self.update_installed_gsx_profiles();
            }

            ui.add_enabled_ui(self.selected_profile.is_some(), |ui| {
                if ui.button("Delete selected Profile").clicked() {
                    let file_location = &self.selected_profile.clone().unwrap().file_location;
                    filehandler::delete_config_file(file_location);
                    self.selected_profile = None;
                    self.update_installed_gsx_profiles();
                }
            });
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

        egui::TopBottomPanel::top(Id::new("top_panel")).show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    self.update_menu_bar_panel(ui);
                });
            });
        });

        egui::SidePanel::left(Id::new("map_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width((content_width / 2.0) - 5.0)
            .show(ctx, |ui| {
                self.update_map_panel(ui);
            });

        egui::SidePanel::right(Id::new("configlist_panel"))
            .frame(rimless)
            .resizable(false)
            .exact_width((content_width / 2.0) - 5.0)
            .show(ctx, |ui| {
                self.update_table_panel(ui);
            });
    }
}

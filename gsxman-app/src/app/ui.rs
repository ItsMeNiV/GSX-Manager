use super::GsxmanApp;
use crate::core::{constants, filehandler};
use egui::{menu, Color32, Id, Margin, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use walkers::{
    extras::{Place, Places, Style},
    Map, Plugin, Position,
};

pub struct ClickWatcher {
    pub places: Option<Vec<Place>>,
    pub clicked_icao: Option<String>,
    pub has_clicked: bool,
}

struct GsxPlace(Place);

impl GsxPlace {
    pub fn to_place(&self) -> Place {
        Place {
            position: self.0.position.clone(),
            label: self.0.label.clone(),
            symbol: self.0.symbol.clone(),
            style: self.0.style.clone(),
        }
    }
}

impl Clone for GsxPlace {
    fn clone(&self) -> Self {
        Self {
            0: Place {
                position: self.0.position.clone(),
                label: self.0.label.clone(),
                symbol: self.0.symbol.clone(),
                style: self.0.style.clone(),
            },
        }
    }
}

impl Plugin for &mut ClickWatcher {
    fn run(
        &mut self,
        response: &egui::Response,
        _painter: egui::Painter,
        projector: &walkers::Projector,
    ) {
        let click_position =
            if !response.changed() && response.clicked_by(egui::PointerButton::Primary) {
                response.interact_pointer_pos()
            } else {
                None
            };

        if let Some(click_pos) = click_position {
            self.has_clicked = true;
            self.clicked_icao = None;

            if let Some(places) = &self.places {
                places.iter().for_each(|p| {
                    let airport_position = projector.project(p.position);
                    let offset = 10.0;
                    if click_pos.x > (airport_position.x - offset)
                        && click_pos.x < (airport_position.x + offset)
                        && click_pos.y > (airport_position.y - offset)
                        && click_pos.y < (airport_position.y + offset)
                    {
                        self.clicked_icao = Some(p.label.to_owned());
                    }
                });
            }
        }
    }
}

impl GsxmanApp {
    fn update_map_panel(&mut self, ui: &mut Ui) {
        let places: Vec<GsxPlace> = self
            .installed_gsx_profiles
            .iter()
            .map(|profile| {
                GsxPlace(Place {
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
            })
            .collect();

        let places_copy: Vec<Place> = places.to_vec().iter().map(|p| p.to_place()).collect();
        let places: Vec<Place> = places.iter().map(|p| p.to_place()).collect();

        self.click_watcher.places = Some(places_copy);

        let places = Places::new(places);

        // Manual Zoom by Scrolling. Map Library only allows Zooming by holding Ctrl
        if ui.rect_contains_pointer(ui.max_rect()) {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta);
            if scroll_delta.y > 0.0 {
                match self.map_memory.zoom_in() {
                    Ok(_) => {}
                    Err(_) => {}
                };
            } else if scroll_delta.y < 0.0 {
                match self.map_memory.zoom_out() {
                    Ok(_) => {}
                    Err(_) => {}
                };
            }
        }

        ui.add(
            Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lat_lon(52.0, 0.0),
            )
            .zoom_gesture(false)
            .with_plugin(places)
            .with_plugin(&mut self.click_watcher),
        );

        if self.click_watcher.has_clicked {
            if let Some(clicked_icao) = &self.click_watcher.clicked_icao {
                self.installed_gsx_profiles.iter().for_each(|p| {
                    if clicked_icao.to_owned() == p.airport.icao {
                        self.selected_profile = Some(p.clone());
                    }
                });
            } else {
                self.selected_profile = None;
            }

            self.click_watcher.has_clicked = false;
        }
    }

    fn update_table_panel(&mut self, ui: &mut Ui) {
        ui.heading("Installed GSX Profiles");
        ui.separator();
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().clip(false))
            .column(Column::auto().clip(false))
            .column(Column::auto().clip(false))
            .column(Column::remainder().clip(false));

        table = table.sense(egui::Sense::click());

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.add(egui::Label::new(RichText::new("ICAO").heading()).selectable(false));
                });
                header.col(|ui| {
                    ui.add(
                        egui::Label::new(RichText::new("Airport Name").heading()).selectable(false),
                    );
                });
                header.col(|ui| {
                    ui.add(
                        egui::Label::new(RichText::new("File Location").heading())
                            .selectable(false),
                    );
                });
            })
            .body(|mut body| {
                for profile in &self.installed_gsx_profiles {
                    body.row(30.0, |mut row| {
                        if let Some(selected_profile) = &self.selected_profile {
                            row.set_selected(selected_profile.airport.icao == profile.airport.icao);
                        }

                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(profile.airport.icao.to_string())
                                    .selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(profile.airport.name.to_string())
                                    .selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(
                                    profile.file_location.as_os_str().to_str().unwrap(),
                                )
                                .selectable(false),
                            );
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
            inner_margin: Margin::symmetric(5.0, 5.0),
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

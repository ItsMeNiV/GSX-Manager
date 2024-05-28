use egui::{Color32, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use itertools::Itertools;
use walkers::Position;

use crate::{app::GsxmanApp, core::filehandler};

use super::{map_panel, UIState};

pub fn update_table_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    match app.ui_state {
        UIState::Overview => update_overview_table(app, ui),
        UIState::Details => update_detail_table(app, ui),
        UIState::SectionDetails => update_section_detail_table(app, ui),
    }
}

fn update_section_detail_table(app: &mut GsxmanApp, ui: &mut Ui) {
    let selected_profile = app.get_selected_profile().unwrap();
    let selected_section = app.get_selected_section().unwrap();
    ui.heading(format!(
        "Section {} Details in {}",
        selected_section.name, selected_profile.file_name
    ));
    ui.separator();
    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().clip(false))
        .column(Column::auto().clip(false))
        .column(Column::remainder().clip(false));

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Label").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Latitude").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Longitude").heading()).selectable(false));
            });
        })
        .body(|mut body| {
            if let Some(pushback_position_left) = &selected_section.pushback_position_left {
                if let Some(pushback_label_left) = &selected_section.pushback_label_left {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(pushback_label_left.to_owned()).selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(pushback_position_left.lat().to_string())
                                    .selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(pushback_position_left.lon().to_string())
                                    .selectable(false),
                            );
                        });
                    });
                }
            }
            if let Some(pushback_position_right) = &selected_section.pushback_position_right {
                if let Some(pushback_label_right) = &selected_section.pushback_label_right {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(pushback_label_right.to_owned()).selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(pushback_position_right.lat().to_string())
                                    .selectable(false),
                            );
                        });
                        row.col(|ui| {
                            ui.add(
                                egui::Label::new(pushback_position_right.lon().to_string())
                                    .selectable(false),
                            );
                        });
                    });
                }
            }
        });
}

fn update_detail_table(app: &mut GsxmanApp, ui: &mut Ui) {
    let selected_profile = app.get_selected_profile().unwrap();
    ui.heading(format!(
        "Details {} by {}",
        selected_profile.file_name,
        selected_profile.profile_data.as_ref().unwrap().creator
    ));
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

    let mut clicked_section_id = None;
    let mut new_ui_state = None;

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Locationname").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Latitude").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Longitude").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Actions").heading()).selectable(false));
            });
        })
        .body(|mut body| {
            let sections = selected_profile
                .profile_data
                .as_ref()
                .unwrap()
                .sections
                .clone();
            let sections_iter = sections.iter().sorted_by(|a, b| Ord::cmp(&a.name, &b.name));
            for section in sections_iter {
                body.row(30.0, |mut row| {
                    if let Some(selected_section_id) = app.selected_section_id.as_ref() {
                        row.set_selected(*selected_section_id == section.id);
                    }

                    row.col(|ui| {
                        ui.add(egui::Label::new(section.name.to_string()).selectable(false));
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(section.position.lat().to_string()).selectable(false),
                        );
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(section.position.lon().to_string()).selectable(false),
                        );
                    });
                    row.col(|ui| {
                        if ui.button("Show associated Positions").clicked() {
                            clicked_section_id = {
                                let id;
                                if let Some(selected_section_id) = app.selected_section_id {
                                    if selected_section_id == section.id.clone() {
                                        id = None;
                                    } else {
                                        id = Some(section.id.clone())
                                    }
                                } else {
                                    id = Some(section.id.clone())
                                }
                                id
                            };
                            new_ui_state = Some(UIState::SectionDetails);
                        }
                    });

                    if row.response().clicked() {
                        clicked_section_id = Some(section.id.clone());
                    }
                });
            }
        });

    if let Some(clicked_section_id) = clicked_section_id {
        if let Some(selected_section_id) = app.selected_section_id.as_ref() {
            if *selected_section_id == clicked_section_id {
                app.selected_section_id = None
            } else {
                app.selected_section_id = Some(clicked_section_id.clone());
            }
        } else {
            app.selected_section_id = Some(clicked_section_id.clone());
        }
    }
    if let Some(new_ui_state) = new_ui_state {
        app.ui_state = new_ui_state;

        match app.ui_state {
            UIState::SectionDetails => {
                if let Some(selected_section) = app.get_selected_section() {
                    let zoom_pos = Position::from_lat_lon(
                        selected_section.position.lat(),
                        selected_section.position.lon(),
                    );
                    map_panel::zoom_map_to_position(app, zoom_pos, 2);
                }
            }
            _ => {}
        }
    }
}

fn update_overview_table(app: &mut GsxmanApp, ui: &mut Ui) {
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
                ui.add(egui::Label::new(RichText::new("Airport Name").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(
                    egui::Label::new(RichText::new("File Location").heading()).selectable(false),
                );
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Actions").heading()).selectable(false));
            });
        })
        .body(|mut body| {
            let mut installed_profiles = app.installed_gsx_profiles.clone();
            let installed_profiles_iter = installed_profiles
                .iter_mut()
                .sorted_by(|a, b| Ord::cmp(&a.1.airport.icao, &b.1.airport.icao));

            for (id, profile) in installed_profiles_iter {
                body.row(30.0, |mut row| {
                    if let Some(selected_profile_id) = app.selected_profile_id {
                        row.set_selected(selected_profile_id == *id);
                    }

                    let icao_string = {
                        if profile.has_duplicate_error {
                            RichText::new(String::from("⚠ ") + profile.airport.icao.as_str()).color(Color32::RED)
                        } else {
                            RichText::new(profile.airport.icao.to_string())
                        }
                    };

                    row.col(|ui| {
                        let response = ui.add(
                            egui::Label::new(icao_string).selectable(false),
                        );
                        if profile.has_duplicate_error && response.hovered() {
                            response.on_hover_text("There is a duplicate Profile of the Same airport. Consider deleting one of them.");
                        }
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(profile.airport.name.to_string()).selectable(false),
                        );
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(profile.file_location.as_os_str().to_str().unwrap())
                                .selectable(false),
                        );
                    });
                    row.col(|ui| {
                        if ui.button("Delete Profile").clicked() {
                            app.selected_profile_id = Some(profile.id.clone());
                            handle_profile_delete(app);
                        }
                        if ui.button("Details").clicked() {
                            app.selected_profile_id = Some(profile.id.clone());
                            handle_profile_details(app);
                        }
                    });

                    if row.response().clicked() {
                        if let Some(selected_profile_id) = app.selected_profile_id {
                            if selected_profile_id == profile.id {
                                app.selected_profile_id = None
                            } else {
                                app.selected_profile_id = Some(profile.id.clone());
                            }
                        } else {
                            app.selected_profile_id = Some(profile.id.clone());
                        }
                    }
                });
            }
        });
}

fn handle_profile_delete(app: &mut GsxmanApp) {
    let file_location = &app.get_selected_profile().unwrap().file_location;
    if filehandler::delete_config_file(file_location) {
        app.selected_profile_id = None;
        app.update_installed_gsx_profiles();
    }
}

fn handle_profile_details(app: &mut GsxmanApp) {
    if let Some(profile) = app.get_selected_profile_mut() {
        filehandler::load_profile_data(profile);

        if let Some(selected_profile) = app.get_selected_profile() {
            let zoom_pos = Position::from_lat_lon(
                selected_profile.airport.location.latitude(),
                selected_profile.airport.location.longitude(),
            );
            map_panel::zoom_map_to_position(app, zoom_pos, 4);
        }

        app.ui_state = UIState::Details;
    }
}

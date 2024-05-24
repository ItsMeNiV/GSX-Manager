use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::app::GsxmanApp;

use super::UIState;

pub fn update_table_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    match app.ui_state {
        UIState::Overview => update_overview_table(app, ui),
        UIState::Details => update_detail_table(app, ui)
    }
    
}

fn update_detail_table(app: &mut GsxmanApp, ui: &mut Ui) {
    let selected_profile = app.get_selected_profile().unwrap();
    ui.heading(format!("Details {} by {}", selected_profile.file_name, selected_profile.profile_data.as_ref().unwrap().creator));
    ui.separator();
    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().clip(false))
        .column(Column::auto().clip(false))
        .column(Column::auto().clip(false))
        .column(Column::remainder().clip(false));

    //table = table.sense(egui::Sense::click());

    table
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Locationname").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Latitude").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(
                    egui::Label::new(RichText::new("Longitude").heading()).selectable(false),
                );
            });
        })
        .body(|mut body| {
            
            for section in selected_profile.profile_data.as_ref().unwrap().sections.iter() {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(section.name.to_string()).selectable(false),
                        );
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(section.position.lat().to_string()).selectable(false),
                        );
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(section.position.lon().to_string())
                                .selectable(false),
                        );
                    });
                });
            }
        });
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
        })
        .body(|mut body| {
            for (id, profile) in app.installed_gsx_profiles.iter() {
                body.row(30.0, |mut row| {
                    if let Some(selected_profile_id) = app.selected_profile_id {
                        row.set_selected(selected_profile_id == *id);
                    }

                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(profile.airport.icao.to_string()).selectable(false),
                        );
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
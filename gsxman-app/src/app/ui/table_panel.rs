use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::app::GsxmanApp;

pub fn update_table_panel(app: &mut GsxmanApp, ui: &mut Ui) {
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
            for profile in &app.installed_gsx_profiles {
                body.row(30.0, |mut row| {
                    if let Some(selected_profile) = &app.selected_profile {
                        row.set_selected(selected_profile.airport.icao == profile.airport.icao);
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
                        if let Some(selected_profile) = &app.selected_profile {
                            if selected_profile.airport.icao == profile.airport.icao {
                                app.selected_profile = None
                            } else {
                                app.selected_profile = Some(profile.clone());
                            }
                        } else {
                            app.selected_profile = Some(profile.clone());
                        }
                    }
                });
            }
        });
}

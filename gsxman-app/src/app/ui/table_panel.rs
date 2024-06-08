use egui::{Align, Color32, RichText, Ui};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use itertools::Itertools;

use crate::app::GsxmanApp;

use super::{filter_profile_details, filter_profiles, UIState};

pub fn update_table_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    StripBuilder::new(ui)
        .size(Size::exact(25.0))
        .size(Size::remainder())
        .vertical(|mut strip| {
            strip.cell(|ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut app.filter_text).hint_text("Filter"));
                    if ui.button("Clear").clicked() {
                        app.filter_text.clear();
                    };
                });
            });
            strip.cell(|ui| {
                egui::ScrollArea::horizontal().auto_shrink([false, false]).show(ui, |ui| match app.ui_state {
                    UIState::Overview => update_overview_table(app, ui),
                    UIState::Details => update_detail_table(app, ui),
                    UIState::SectionDetails => update_section_detail_table(app, ui),
                });
            });
        });
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
            let filter_text = app.filter_text.clone().to_lowercase();
            let filter_text_str = filter_text.as_str();
            if let Some(pushback_position_left) = &selected_section.pushback_position_left {
                if let Some(pushback_label_left) = &selected_section.pushback_label_left {
                    if pushback_label_left.to_lowercase().contains(filter_text_str) {
                        body.row(40.0, |mut row| {
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(pushback_label_left.to_owned())
                                        .selectable(false),
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
            }
            if let Some(pushback_position_right) = &selected_section.pushback_position_right {
                if let Some(pushback_label_right) = &selected_section.pushback_label_right {
                    if pushback_label_right
                        .to_lowercase()
                        .contains(filter_text_str)
                    {
                        body.row(40.0, |mut row| {
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(pushback_label_right.to_owned())
                                        .selectable(false),
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
            }
        });
}

fn update_detail_table(app: &mut GsxmanApp, ui: &mut Ui) {
    let selected_profile = app.get_selected_profile().unwrap();
    ui.heading(format!(
        "Details {} by {}",
        selected_profile.file_name, selected_profile.creator
    ));
    ui.separator();
    let mut table = TableBuilder::new(ui)
        .striped(true)
        .resizable(false)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().clip(false))
        .column(Column::auto().clip(false))
        .column(Column::remainder().clip(false));

    table = table.sense(egui::Sense::click());

    if let Some(scroll_to_row) = &app.scroll_to_row {
        table = table.scroll_to_row(*scroll_to_row, Some(Align::Center));
    }

    let mut clicked_section_id = None;

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
        })
        .body(|mut body| {
            let sections = selected_profile
                .profile_data
                .as_ref()
                .unwrap()
                .sections
                .clone();
            let filter_text = app.filter_text.clone();
            let sections_iter = sections
                .iter()
                .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
                .filter(|&section| filter_profile_details(&filter_text, section));
            for section in sections_iter {
                body.row(40.0, |mut row| {
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

                    if row.response().clicked() {
                        clicked_section_id = Some(section.id);
                    }
                });
            }
        });

    if let Some(clicked_section_id) = clicked_section_id {
        if let Some(selected_section_id) = app.selected_section_id.as_ref() {
            if *selected_section_id == clicked_section_id {
                app.selected_section_id = None
            } else {
                app.selected_section_id = Some(clicked_section_id);
            }
        } else {
            app.selected_section_id = Some(clicked_section_id);
        }
    }
    app.scroll_to_row = None;
}

fn update_overview_table(app: &mut GsxmanApp, ui: &mut Ui) {
    ui.heading("Installed GSX Profiles");
    ui.separator();
    let mut table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto().clip(false)) //ICAO
        .column(Column::auto().clip(true)) //Airport Name
        .column(Column::initial(400.0).clip(true)) //File Location
        .column(Column::auto().clip(false)) //Creator
        .column(Column::remainder().clip(false)); //Last Modified

    table = table.sense(egui::Sense::click());

    if let Some(scroll_to_row) = app.scroll_to_row {
        table = table.scroll_to_row(scroll_to_row, Some(Align::Center));
    }

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
                ui.add(egui::Label::new(RichText::new("Creator").heading()).selectable(false));
            });
            header.col(|ui| {
                ui.add(egui::Label::new(RichText::new("Last modified").heading()).selectable(false));
            });
        })
        .body(|mut body| {
            let filter_text = app.filter_text.clone();
            let installed_profiles = app.installed_gsx_profiles.clone();
            let installed_profiles_iter = installed_profiles
                .iter()
                .sorted_by(|a, b| Ord::cmp(&a.1.airport.icao, &b.1.airport.icao))
                .filter(|&(_, profile)| filter_profiles(&filter_text, profile));

            for (id, profile) in installed_profiles_iter {
                body.row(40.0, |mut row| {
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
                        ui.add(
                            egui::Label::new(profile.creator.to_string())
                                .selectable(false),
                        );
                    });
                    row.col(|ui| {
                        ui.add(
                            egui::Label::new(profile.last_modified.format("%d/%m/%Y %T").to_string())
                                .selectable(false),
                        );
                    });

                    if row.response().clicked() {
                        if let Some(selected_profile_id) = app.selected_profile_id {
                            if selected_profile_id == profile.id {
                                app.selected_profile_id = None
                            } else {
                                app.selected_profile_id = Some(profile.id);
                            }
                        } else {
                            app.selected_profile_id = Some(profile.id);
                        }
                    }
                });
            }
        });
    app.scroll_to_row = None;
}

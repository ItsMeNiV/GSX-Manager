use egui::{menu, Ui};
use walkers::Position;

use crate::{app::GsxmanApp, core::filehandler};
use crate::app::ui::UIState;

use super::map_panel;

pub fn update_menu_bar_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    menu::bar(ui, |ui| {
        match app.ui_state {
            UIState::Overview => {
                if ui.button("Import new Profile").clicked() {
                    filehandler::import_config_file_dialog();
                    app.update_installed_gsx_profiles(true);
                }

                let selected_profile = app.get_selected_profile();
                if ui.add_enabled(selected_profile.is_some(), egui::Button::new("Delete Profile")).clicked() {
                    handle_profile_delete(app);
                    app.filter_text.clear();
                }

                let selected_profile = app.get_selected_profile();
                if ui.add_enabled(selected_profile.is_some(), egui::Button::new("Profile Details")).clicked() {
                    handle_profile_details(app);
                    app.filter_text.clear();
                }

                let selected_profile = app.get_selected_profile();
                if ui.add_enabled(selected_profile.is_some(), egui::Button::new("Profile Notes")).clicked() {
                    handle_profile_notes(app);
                    app.filter_text.clear();
                }
            }
            UIState::Details => {
                if ui.button("Back to Overview").clicked() {
                    app.selected_section_id = None;
                    app.ui_state = UIState::Overview;
                    app.filter_text.clear();
                    if let Some(selected_profile) = app.get_selected_profile_mut() {
                        selected_profile.profile_data = None;
                    }
                }

                let selected_section = app.get_selected_section();
                if ui.add_enabled(selected_section.is_some(), egui::Button::new("Show associated Positions")).clicked() {
                    handle_section_details(app);
                    app.filter_text.clear();
                }
            },
            UIState::SectionDetails => {
                if ui.button("Back to Profile Details").clicked() {
                    app.ui_state = UIState::Details;
                    app.filter_text.clear();
                }
            },
            UIState::Notes => {
                if ui.button("Back to Overview").clicked() {
                    app.ui_state = UIState::Overview;
                    app.filter_text.clear();

                    let selected_profile = app.get_selected_profile().unwrap().clone();
                    let profile_file_location = selected_profile.file_location.as_os_str().to_str().unwrap();
                    app.user_data[profile_file_location]["notes"] = selected_profile.notes.into();
                    filehandler::write_user_data(&app.user_data);
                }
            }
        };
    });
}

fn handle_profile_delete(app: &mut GsxmanApp) {
    let file_location = &app.get_selected_profile().unwrap().file_location;
    if filehandler::delete_config_file(file_location) {
        app.selected_profile_id = None;
        app.update_installed_gsx_profiles(false);
    }
}

fn handle_profile_details(app: &mut GsxmanApp) {
    if let Some(profile) = app.get_selected_profile_mut() {
        if profile.profile_data.is_none() {
            filehandler::load_profile_data(profile);
        }

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

fn handle_section_details(app: &mut GsxmanApp) {
    if let Some(selected_section) = app.get_selected_section() {
        let zoom_pos = Position::from_lat_lon(
            selected_section.position.lat(),
            selected_section.position.lon(),
        );
        map_panel::zoom_map_to_position(app, zoom_pos, 2);
        
        app.ui_state = UIState::SectionDetails;
    }
}

fn handle_profile_notes(app: &mut GsxmanApp) {
    let user_data = app.user_data.clone();
    if let Some(profile) = app.get_selected_profile_mut() {
        if profile.profile_data.is_none() {
            filehandler::load_profile_data(profile);
        }

        let profile_file_location = profile.file_location.as_os_str().to_str().unwrap();
        if user_data[profile_file_location] != json::Null {
            profile.notes = user_data[profile_file_location]["notes"].to_string();
        }

        if let Some(selected_profile) = app.get_selected_profile() {
            let zoom_pos = Position::from_lat_lon(
                selected_profile.airport.location.latitude(),
                selected_profile.airport.location.longitude(),
            );
            map_panel::zoom_map_to_position(app, zoom_pos, 4);
        }

        app.ui_state = UIState::Notes;
    }
}
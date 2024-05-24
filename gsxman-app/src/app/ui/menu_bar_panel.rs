use egui::{menu, Ui};
use walkers::Position;

use crate::{app::GsxmanApp, core::filehandler};
use crate::app::ui::{map_panel, UIState};

pub fn update_menu_bar_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    menu::bar(ui, |ui| {
        match app.ui_state {
            UIState::Overview => {
                if ui.button("Import Profile").clicked() {
                    filehandler::import_config_file_dialog();
                    app.update_installed_gsx_profiles();
                }

                ui.add_enabled_ui(app.selected_profile_id.is_some(), |ui| {
                    if ui.button("See Profile Details").clicked() {
                        let profile = app.get_selected_profile_mut().unwrap();
                        filehandler::load_profile_data(profile);

                        if let Some(selected_profile) = app.get_selected_profile() {
                            let zoom_pos = Position::from_lat_lon(selected_profile.airport.location.latitude(), selected_profile.airport.location.longitude());
                            map_panel::zoom_map_to_position(app, zoom_pos);
                        }

                        app.ui_state = UIState::Details;
                    }
                });

                ui.add_enabled_ui(app.selected_profile_id.is_some(), |ui| {
                    if ui.button("Delete selected Profile").clicked() {
                        let file_location = &app.get_selected_profile().unwrap().file_location;
                        filehandler::delete_config_file(file_location);
                        app.selected_profile_id = None;
                        app.update_installed_gsx_profiles();
                    }
                });
            }
            UIState::Details => {
                if ui.button("Back to Overview").clicked() {
                    app.get_selected_profile_mut().unwrap().profile_data = None;

                    app.ui_state = UIState::Overview;
                }
            }
        };
    });
}

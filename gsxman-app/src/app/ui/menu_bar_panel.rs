use egui::{menu, Ui};

use crate::{app::GsxmanApp, core::filehandler};
use crate::app::ui::UIState;

pub fn update_menu_bar_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    menu::bar(ui, |ui| {
        match app.ui_state {
            UIState::Overview => {
                if ui.button("Import Profile").clicked() {
                    filehandler::import_config_file_dialog();
                    app.update_installed_gsx_profiles();
                }
                
                ui.add_enabled_ui(app.selected_profile.is_some(), |ui| {
                    if ui.button("See Profile Details").clicked() {
                        let profile = &mut app.selected_profile.clone().unwrap();
                        filehandler::load_profile_data(profile);

                        for p in app.installed_gsx_profiles.iter_mut() {
                            if p.file_location == profile.file_location {
                                p.profile_data = profile.profile_data.clone();
                            }
                        }

                        app.ui_state = UIState::Details;
                    }
                });

                ui.add_enabled_ui(app.selected_profile.is_some(), |ui| {
                    if ui.button("Delete selected Profile").clicked() {
                        let file_location = &app.selected_profile.clone().unwrap().file_location;
                        filehandler::delete_config_file(file_location);
                        app.selected_profile = None;
                        app.update_installed_gsx_profiles();
                    }
                });
            }
            UIState::Details => {}
        };
    });
}

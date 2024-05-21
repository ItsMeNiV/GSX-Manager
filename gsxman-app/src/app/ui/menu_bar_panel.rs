use egui::{menu, Ui};

use crate::{app::GsxmanApp, core::filehandler};

impl GsxmanApp {
    pub fn update_menu_bar_panel(&mut self, ui: &mut Ui) {
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

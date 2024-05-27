use egui::{menu, Ui};

use crate::{app::GsxmanApp, core::filehandler};
use crate::app::ui::UIState;

pub fn update_menu_bar_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    menu::bar(ui, |ui| {
        match app.ui_state {
            UIState::Overview => {
                if ui.button("Import new Profile").clicked() {
                    filehandler::import_config_file_dialog();
                    app.update_installed_gsx_profiles();
                }
            }
            UIState::Details => {
                if ui.button("Back to Overview").clicked() {
                    app.get_selected_profile_mut().unwrap().profile_data = None;

                    app.selected_section_id = None;
                    app.ui_state = UIState::Overview;
                }
            },
            UIState::SectionDetails => {
                if ui.button("Back to Profile Details").clicked() {
                    app.ui_state = UIState::Details;
                }
            }
        };
    });
}

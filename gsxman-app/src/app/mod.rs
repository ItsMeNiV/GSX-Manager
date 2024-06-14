use std::collections::HashMap;

use eframe::egui;
use egui::{Context, Style, Vec2, Visuals};
use json::JsonValue;
use uuid::Uuid;
use walkers::{sources, MapMemory, Tiles};

use GsxmanCore::{constants, Airport, ProfileFile};

use crate::app::ui::UIState;
use crate::core::{self as GsxmanCore, GsxSection};

use self::ui::plugins::ClickWatcher;

mod ui;

struct GsxmanApp {
    map_memory: MapMemory,
    tiles: Tiles,
    installed_gsx_profiles: HashMap<Uuid, ProfileFile>,
    airport_data: HashMap<String, Airport>,
    user_data: JsonValue,
    click_watcher: ui::plugins::ClickWatcher,
    selected_profile_id: Option<Uuid>,
    selected_section_id: Option<Uuid>,
    scroll_to_row: Option<usize>,
    ui_state: UIState,
    filter_text: String,
}

impl GsxmanApp {
    fn new(egui_ctx: Context) -> Self {
        let airport_data = GsxmanCore::filehandler::get_airport_data();
        let map_memory = MapMemory::default();
        let user_data = GsxmanCore::filehandler::get_user_data();
        Self {
            map_memory,
            tiles: Tiles::new(sources::OpenStreetMap, egui_ctx),
            installed_gsx_profiles: GsxmanCore::filehandler::get_installed_gsx_profiles(
                &airport_data,
            ),
            airport_data,
            user_data,
            click_watcher: ClickWatcher {
                places: None,
                clicked_label: None,
                has_clicked: false,
            },
            selected_profile_id: None,
            selected_section_id: None,
            scroll_to_row: None,
            ui_state: UIState::Overview,
            filter_text: String::new(),
        }
    }

    fn update_installed_gsx_profiles(&mut self, profile_added: bool) {
        let profiles_in_folder =
            GsxmanCore::filehandler::get_installed_gsx_profiles(&self.airport_data);

        if profile_added {
            for (id, profile) in profiles_in_folder.iter() {
                let mut profile_exists = false;
                for installed_profile in self.installed_gsx_profiles.values().into_iter() {
                    if installed_profile.file_name == profile.file_name {
                        profile_exists = true;
                    }
                }

                if !profile_exists {
                    self.installed_gsx_profiles
                        .insert(id.clone(), profile.clone());
                }
            }
        } else {
            self.installed_gsx_profiles.retain(|_, profile| {
                for profile_in_folder in profiles_in_folder.values().into_iter() {
                    if profile.file_name == profile_in_folder.file_name {
                        return true;
                    }
                }
                return false;
            });
        }
    }

    fn get_selected_profile(&self) -> Option<&ProfileFile> {
        let mut selected_profile: Option<&ProfileFile> = None;
        if let Some(id) = self.selected_profile_id {
            selected_profile = self.installed_gsx_profiles.get(&id)
        }
        selected_profile
    }

    fn get_selected_profile_mut(&mut self) -> Option<&mut ProfileFile> {
        let mut selected_profile: Option<&mut ProfileFile> = None;
        if let Some(id) = self.selected_profile_id {
            selected_profile = self.installed_gsx_profiles.get_mut(&id)
        }
        selected_profile
    }

    fn get_selected_section(&self) -> Option<&GsxSection> {
        let mut selected_section: Option<&GsxSection> = None;
        if let Some(id) = self.selected_section_id {
            if let Some(profile) = self.get_selected_profile() {
                if let Some(profile_data) = &profile.profile_data {
                    for section in profile_data.sections.iter() {
                        if section.id == id {
                            selected_section = Some(section);
                            break;
                        }
                    }
                }
            }
        }
        selected_section
    }
}

pub fn start_app() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(constants::WINDOW_SIZE)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../../res/icon.png")[..])
                    .expect("Failed to load icon"),
            )
            .with_min_inner_size(Vec2::new(1400.0, 500.0)),
        ..Default::default()
    };

    eframe::run_native(
        "GSXManager",
        options,
        Box::new(|cc| {
            let style = Style {
                visuals: Visuals::dark(),
                ..Style::default()
            };
            cc.egui_ctx.set_style(style);
            let mut app = GsxmanApp::new(cc.egui_ctx.to_owned());
            for _ in 0..12 {
                app.map_memory.zoom_out().expect("Couldn't zoom out");
            }
            Box::<GsxmanApp>::new(app)
        }),
    )
}

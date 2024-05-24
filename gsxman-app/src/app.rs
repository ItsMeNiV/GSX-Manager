use std::collections::HashMap;

use eframe::egui;
use egui::{Context, Style, Visuals};
use uuid::Uuid;
use walkers::{MapMemory, sources, Tiles};

use GsxmanCore::{Airport, constants, ProfileFile};

use crate::app::ui::UIState;
use crate::core as GsxmanCore;

use self::ui::plugins::ClickWatcher;

mod ui;

/// Configuration of the App, this will be saved locally and loaded on Application start (if available)
pub struct AppConfig {
    pub msfs_windowsstore: bool,
    pub gsx_profile_path: Option<String>,
}

struct GsxmanApp {
    _app_config: AppConfig,
    map_memory: MapMemory,
    tiles: Tiles,
    installed_gsx_profiles: HashMap<Uuid, ProfileFile>,
    airport_data: HashMap<String, Airport>,
    click_watcher: ui::plugins::ClickWatcher,
    selected_profile_id: Option<Uuid>,
    ui_state: UIState,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            msfs_windowsstore: true,
            gsx_profile_path: None,
        }
    }
}

impl GsxmanApp {
    fn new(app_config: AppConfig, egui_ctx: Context) -> Self {
        let airport_data = GsxmanCore::filehandler::get_airport_data();
        let map_memory = MapMemory::default();
        Self {
            _app_config: app_config,
            map_memory,
            tiles: Tiles::new(sources::OpenStreetMap, egui_ctx),
            installed_gsx_profiles: GsxmanCore::filehandler::get_installed_gsx_profiles(
                &airport_data,
            ),
            airport_data,
            click_watcher: ClickWatcher {
                places: None,
                clicked_icao: None,
                has_clicked: false,
            },
            selected_profile_id: None,
            ui_state: UIState::Overview,
        }
    }

    fn update_installed_gsx_profiles(&mut self) {
        self.installed_gsx_profiles =
            GsxmanCore::filehandler::get_installed_gsx_profiles(&self.airport_data);
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
}

pub fn start_app(app_config: AppConfig) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(constants::WINDOW_SIZE)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../res/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
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
            let mut app = GsxmanApp::new(app_config, cc.egui_ctx.to_owned());
            for _ in 0..12 {
                app.map_memory.zoom_out().expect("Couldn't zoom out");
            }
            Box::<GsxmanApp>::new(app)
        }),
    )
}

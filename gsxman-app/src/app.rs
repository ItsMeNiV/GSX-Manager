use std::collections::HashMap;
use crate::core as GsxmanCore;
use GsxmanCore::{constants, Airport, ConfigFile};
use eframe::egui;
use egui::{Context, Style, Visuals};
use walkers::{sources, MapMemory, Tiles};

use self::ui::ClickWatcher;

mod ui;

/// Configuration of the App, this will be saved locally and loaded on Application start (if available)
pub struct AppConfig {
    pub msfs_windowsstore: bool,
    pub gsx_profile_path: Option<String>,
}

struct GsxmanApp {
    app_config: AppConfig,
    tiles: Tiles,
    map_memory: MapMemory,
    installed_gsx_profiles: Vec<ConfigFile>,
    airport_data: HashMap<String, Airport>,
    click_watcher: ui::ClickWatcher,
    selected_profile: Option<ConfigFile>
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
        Self {
            app_config,
            tiles: Tiles::new(sources::OpenStreetMap, egui_ctx),
            map_memory: MapMemory::default(),
            installed_gsx_profiles: GsxmanCore::filehandler::get_installed_gsx_profiles(&airport_data),
            airport_data,
            click_watcher: ClickWatcher::default(),
            selected_profile: None
        }
    }

    fn update_installed_gsx_profiles(&mut self) {
        self.installed_gsx_profiles = GsxmanCore::filehandler::get_installed_gsx_profiles(&self.airport_data);
    }
}

pub fn start_app(app_config: AppConfig) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(constants::WINDOW_SIZE),
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

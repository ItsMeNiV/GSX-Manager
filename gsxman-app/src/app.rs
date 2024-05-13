use std::collections::HashMap;
use crate::core as GsxmanCore;
use GsxmanCore::{constants, Airport, ConfigFile};
use eframe::egui;
use egui::{Context, Style, Visuals};
use walkers::{sources, MapMemory, Tiles};

mod ui;

/// Configuration of the App, this will be saved locally and loaded on Application start (if available)
pub struct AppConfig {
    pub msfs_windowsstore: bool,
}

struct GsxmanApp {
    app_config: AppConfig,
    tiles: Tiles,
    map_memory: MapMemory,
    installed_gsx_profiles: Vec<ConfigFile>,
    airport_data: HashMap<String, Airport>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            msfs_windowsstore: true,
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
            airport_data
        }
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
            Box::<GsxmanApp>::new(GsxmanApp::new(app_config, cc.egui_ctx.to_owned()))
        }),
    )
}

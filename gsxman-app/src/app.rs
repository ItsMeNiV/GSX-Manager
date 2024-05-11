use crate::util;
use eframe::{egui, App};

pub struct Paths {
    pub gsxprofiles: String,
    pub communityfolder: String
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            gsxprofiles: match util::get_gsx_profile_path().to_str() {Some(val) => String::from(val), _ => String::from("")},
            communityfolder: Default::default()
        }
    }
}

/// Configuration of the App, this will be saved locally and loaded on Application start (if available)
 pub struct AppConfig {
    pub msfs_windowsstore: bool,
    pub paths: Paths
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            msfs_windowsstore: true,
            paths: Default::default()
        }
    }
}

struct GsxmanApp {

}

impl Default for GsxmanApp {
    fn default() -> Self {
        Self {  }
    }
}

impl eframe::App for GsxmanApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}

pub fn start_app(config: &AppConfig) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1600.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native(
        "GSXManager",
        options,
        Box::new(|cc| {
            Box::<GsxmanApp>::default()
    }),
    )
}
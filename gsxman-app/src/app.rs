use crate::{core::constants, util};
use eframe::egui;
use egui::{Context, Id, Style, Visuals};
use egui_extras::{TableBuilder, Column};
use walkers::{Tiles, Map, MapMemory, Position, sources::OpenStreetMap};

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
    tiles: Tiles,
    map_memory: MapMemory,
}

impl GsxmanApp {
    fn new(egui_ctx: Context) -> Self {
        Self {
            tiles: Tiles::new(OpenStreetMap, egui_ctx),
            map_memory: MapMemory::default(),
        }
    }
}

impl eframe::App for GsxmanApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let content_width = if let Some(rect) = ctx.input(|i| i.viewport().inner_rect) {
            rect.width()
        } else {
            constants::WINDOW_SIZE.x
        };

        let rimless = egui::Frame {
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        egui::SidePanel::left(Id::new("map_panel"))
        .resizable(false)
        .exact_width(content_width/2.0)
        .show(ctx, |ui| {
            ui.add(Map::new(
                Some(&mut self.tiles),
                &mut self.map_memory,
                Position::from_lon_lat(17.03664, 51.09916)
            ).zoom_gesture(true));
        });

        egui::SidePanel::right(Id::new("configlist_panel"))
        .frame(rimless)
        .resizable(false)
        .exact_width(content_width/2.0)
        .show(ctx, |ui| {
            ui.heading("Installed GSX Profiles");
            TableBuilder::new(ui)
            .striped(true).resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::remainder())
            .column(Column::remainder().resizable(false))
            .header(20.0, |mut header| {
                header.col(|ui| { ui.heading("ICAO"); });
                header.col(|ui| { ui.heading("Airport Name"); });
                header.col(|ui| { ui.heading("File Location"); });
            })
            .body(|mut body| {
                body.row(30.0, |mut row| {
                    row.col(|ui| {ui.label("LSZH");});
                    row.col(|ui| {ui.label("Zürich Airport");});
                    row.col(|ui| {ui.label("C:\\Users\\Yannik\\AppData\\Roaming\\virtuali\\GSX\\MSFS\\lszh-fsdt.ini");});
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {ui.label("LSZH");});
                    row.col(|ui| {ui.label("Zürich Airport");});
                    row.col(|ui| {ui.label("C:\\Users\\Yannik\\AppData\\Roaming\\virtuali\\GSX\\MSFS\\lszh-fsdt.ini");});
                });
                body.row(30.0, |mut row| {
                    row.col(|ui| {ui.label("LSZH");});
                    row.col(|ui| {ui.label("Zürich Airport");});
                    row.col(|ui| {ui.label("C:\\Users\\Yannik\\AppData\\Roaming\\virtuali\\GSX\\MSFS\\lszh-fsdt.ini");});
                });
            });
        });
    }
}

pub fn start_app(config: &AppConfig) -> Result<(), eframe::Error> {
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
            Box::<GsxmanApp>::new(GsxmanApp::new(cc.egui_ctx.to_owned()))
    }),
    )
}
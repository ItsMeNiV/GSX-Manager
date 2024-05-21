use egui::{Color32, Ui};
use walkers::{
    extras::{Place, Places, Style},
    Map, Position,
};

use crate::app::GsxmanApp;

struct GsxPlace(Place);

impl GsxPlace {
    pub fn to_place(&self) -> Place {
        Place {
            position: self.0.position.clone(),
            label: self.0.label.clone(),
            symbol: self.0.symbol.clone(),
            style: self.0.style.clone(),
        }
    }
}

impl Clone for GsxPlace {
    fn clone(&self) -> Self {
        Self {
            0: Place {
                position: self.0.position.clone(),
                label: self.0.label.clone(),
                symbol: self.0.symbol.clone(),
                style: self.0.style.clone(),
            },
        }
    }
}

pub fn update_map_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    let places: Vec<GsxPlace> = app
        .installed_gsx_profiles
        .iter()
        .map(|profile| {
            GsxPlace(Place {
                label: profile.airport.icao.to_owned(),
                position: Position::from_lat_lon(
                    profile.airport.location.latitude(),
                    profile.airport.location.longitude(),
                ),
                symbol: '✈',
                style: Style {
                    label_background: if let Some(selected_profile) = &app.selected_profile {
                        if selected_profile.airport.icao == profile.airport.icao {
                            Color32::BLUE.gamma_multiply(0.8)
                        } else {
                            Color32::BLACK.gamma_multiply(0.8)
                        }
                    } else {
                        Color32::BLACK.gamma_multiply(0.8)
                    },
                    symbol_background: if let Some(selected_profile) = &app.selected_profile {
                        if selected_profile.airport.icao == profile.airport.icao {
                            Color32::BLUE.gamma_multiply(0.8)
                        } else {
                            Color32::WHITE.gamma_multiply(0.8)
                        }
                    } else {
                        Color32::WHITE.gamma_multiply(0.8)
                    },
                    ..Default::default()
                },
            })
        })
        .collect();

    let places_copy: Vec<Place> = places.to_vec().iter().map(|p| p.to_place()).collect();
    let places: Vec<Place> = places.iter().map(|p| p.to_place()).collect();

    app.click_watcher.places = Some(places_copy);

    let places = Places::new(places);

    // Manual Zoom by Scrolling. Map Library only allows Zooming by holding Ctrl
    if ui.rect_contains_pointer(ui.max_rect()) {
        let scroll_delta = ui.input(|i| i.raw_scroll_delta);
        if scroll_delta.y > 0.0 {
            match app.map_memory.zoom_in() {
                Ok(_) => {}
                Err(_) => {}
            };
        } else if scroll_delta.y < 0.0 {
            match app.map_memory.zoom_out() {
                Ok(_) => {}
                Err(_) => {}
            };
        }
    }

    ui.add(
        Map::new(
            Some(&mut app.tiles),
            &mut app.map_memory,
            Position::from_lat_lon(52.0, 0.0),
        )
        .zoom_gesture(false)
        .with_plugin(places)
        .with_plugin(&mut app.click_watcher),
    );

    if app.click_watcher.has_clicked {
        if let Some(clicked_icao) = &app.click_watcher.clicked_icao {
            app.installed_gsx_profiles.iter().for_each(|p| {
                if clicked_icao.to_owned() == p.airport.icao {
                    app.selected_profile = Some(p.clone());
                }
            });
        } else {
            app.selected_profile = None;
        }

        app.click_watcher.has_clicked = false;
    }
}

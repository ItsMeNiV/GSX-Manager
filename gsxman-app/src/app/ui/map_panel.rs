use egui::{Color32, Ui, Vec2};
use walkers::{
    extras::{Place, Places, Style},
    Map, Position, Projector,
};

use crate::{app::GsxmanApp, core::GsxPlace};
use crate::app::ui::UIState;

fn handle_scrolling(app: &mut GsxmanApp, ui: &mut Ui) {
    let position = {
        if let Some(position) = app.map_memory.detached() {
            position
        } else {
            Position::from_lat_lon(52.0, 0.0)
        }
    };
    if ui.rect_contains_pointer(ui.max_rect()) {
        let projected_pointer_pos = {
            let rect_center = ui.max_rect().center();
            let pointer_pos = ui.input(|i| i.pointer.interact_pos()).unwrap();
            let offset = Vec2 {
                x: pointer_pos.x - rect_center.x,
                y: pointer_pos.y - rect_center.y,
            };
            let projector = Projector::new(ui.max_rect(), &app.map_memory, position);
            projector.unproject(offset)
        };

        let scroll_delta = ui.input(|i| i.raw_scroll_delta);
        if scroll_delta.y > 0.0 {
            app.map_memory.center_at(projected_pointer_pos);
            match app.map_memory.zoom_in() {
                Ok(_) => {}
                Err(_) => {}
            };
        } else if scroll_delta.y < 0.0 {
            app.map_memory.center_at(projected_pointer_pos);
            match app.map_memory.zoom_out() {
                Ok(_) => {}
                Err(_) => {}
            };
        }
    }
}

pub fn update_map_panel(app: &mut GsxmanApp, ui: &mut Ui) {
    let places = get_places_to_display(app);

    let places_copy: Vec<Place> = places.to_vec().iter().map(|p| p.to_place()).collect();
    let places: Vec<Place> = places.iter().map(|p| p.to_place()).collect();

    app.click_watcher.places = Some(places_copy);

    let places = Places::new(places);

    // Manual Zoom by Scrolling. Map Library only allows Zooming by holding Ctrl
    handle_scrolling(app, ui);

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

    // TODO: Maybe need to rethink how to handle this in case of mutliple installed profiles for same airport
    if app.click_watcher.has_clicked {
        if let Some(clicked_icao) = &app.click_watcher.clicked_icao {
            for (_, profile) in app.installed_gsx_profiles.iter() {
                if clicked_icao.to_owned() == profile.airport.icao {
                    app.selected_profile_id = Some(profile.id.clone());
                }
            }
        } else {
            app.selected_profile_id = None;
        }

        app.click_watcher.has_clicked = false;
    }
}

fn get_places_to_display(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    match app.ui_state {
        UIState::Overview => {
            let mut places: Vec<GsxPlace> = Vec::new();
            for (_, profile) in app.installed_gsx_profiles.iter() {
                places.push(GsxPlace(Place {
                    label: profile.airport.icao.to_owned(),
                    position: Position::from_lat_lon(
                        profile.airport.location.latitude(),
                        profile.airport.location.longitude(),
                    ),
                    symbol: '✈',
                    style: Style {
                        label_background: if let Some(selected_profile) = app.get_selected_profile() {
                            if selected_profile.airport.icao == profile.airport.icao {
                                Color32::BLUE.gamma_multiply(0.8)
                            } else {
                                Color32::BLACK.gamma_multiply(0.8)
                            }
                        } else {
                            Color32::BLACK.gamma_multiply(0.8)
                        },
                        symbol_background: if let Some(selected_profile) = app.get_selected_profile() {
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
                }));
            }
            places
        }
        UIState::Details => {
            vec![]
        }
    }
}

pub fn zoom_map_to_position(app: &mut GsxmanApp, position: Position) {
    app.map_memory.center_at(position);

    // Since we don't have any fine control of the zoom level outside of the library we use this hack
    // First zoom in all the way (until zoom_in() returns Err(InvalidZoom)), then zoom out as far as we need
    while app.map_memory.zoom_in().is_ok() {}
    for _ in 0..4 {
        match app.map_memory.zoom_out() {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}
use egui::{Color32, Ui, Vec2};
use itertools::Itertools;
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

    // TODO: Maybe need to rethink how to handle this in case of multiple installed profiles for same airport
    if app.click_watcher.has_clicked {
        if let Some(clicked_label) = &app.click_watcher.clicked_label {
            match app.ui_state {
                UIState::Overview => {
                    let mut row_index = 0;
                    for (_, profile) in app.installed_gsx_profiles.iter().sorted_by(|a, b| Ord::cmp(&a.1.airport.icao, &b.1.airport.icao)) {
                        if clicked_label.to_owned() == profile.airport.icao {
                            app.selected_profile_id = Some(profile.id.clone());
                            app.scroll_to_row = Some(row_index);
                        }
                        row_index += 1;
                    }
                },
                UIState::Details => {
                    for (_, profile) in app.installed_gsx_profiles.iter() {
                        if let Some(profile_data) = profile.profile_data.as_ref() {
                            let mut row_index = 0;
                            for section in profile_data.sections.iter().sorted_by(|a, b| Ord::cmp(&a.name, &b.name)) {
                                if clicked_label.to_owned() == section.name.to_owned() {
                                    app.selected_section_id = Some(section.id.clone());
                                    app.scroll_to_row = Some(row_index);
                                }
                                row_index += 1;
                            }
                        }
                    }
                },
                UIState::SectionDetails => {}
            }
            
        } else {
            match app.ui_state {
                UIState::Overview => app.selected_profile_id = None,
                UIState::Details => app.selected_section_id = None,
                UIState::SectionDetails => {}
            }
        }

        app.click_watcher.has_clicked = false;
    }
}

fn get_places_to_display(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    match app.ui_state {
        UIState::Overview => get_airport_places(app),
        UIState::Details => get_airport_detail_places(app),
        UIState::SectionDetails => get_section_detail_places(app)
    }
}

fn get_section_detail_places(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    let mut places: Vec<GsxPlace> = vec![];
    if let Some(selected_section) = app.get_selected_section() {
        places.push(GsxPlace(Place {
            label: selected_section.name.to_owned(),
            position: Position::from_lat_lon(
                selected_section.position.lat(),
                selected_section.position.lon(),
            ),
            symbol: '✈',
            style: Style {
                label_background: Color32::BLACK.gamma_multiply(0.8),
                symbol_background: Color32::WHITE.gamma_multiply(0.8),
                ..Default::default()
            },
        }));
        if let Some(pushback_position_left) = &selected_section.pushback_position_left {
            if let Some(pushback_label_left) = &selected_section.pushback_label_left {
                places.push(GsxPlace(Place {
                    label: pushback_label_left.to_owned(),
                    position: Position::from_lat_lon(
                        pushback_position_left.lat(),
                        pushback_position_left.lon(),
                    ),
                    symbol: '🖈',
                    style: Style {
                        label_background: Color32::BLACK.gamma_multiply(0.8),
                        symbol_background: Color32::WHITE.gamma_multiply(0.8),
                        ..Default::default()
                    },
                }));
            }
        }
        if let Some(pushback_position_right) = &selected_section.pushback_position_right {
            if let Some(pushback_label_right) = &selected_section.pushback_label_right {
                places.push(GsxPlace(Place {
                    label: pushback_label_right.to_owned(),
                    position: Position::from_lat_lon(
                        pushback_position_right.lat(),
                        pushback_position_right.lon(),
                    ),
                    symbol: '🖈',
                    style: Style {
                        label_background: Color32::BLACK.gamma_multiply(0.8),
                        symbol_background: Color32::WHITE.gamma_multiply(0.8),
                        ..Default::default()
                    },
                }));
            }
        }
    }
    places
}

fn get_airport_detail_places(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    let mut places: Vec<GsxPlace> = vec![];
    if let Some(profile_data) = &app.get_selected_profile().as_ref().unwrap().profile_data {
        for section in profile_data.sections.iter() {
            places.push(GsxPlace(Place {
                label: section.name.to_owned(),
                position: Position::from_lat_lon(
                    section.position.lat(),
                    section.position.lon(),
                ),
                symbol: '🖈',
                style: Style {
                    label_background: if let Some(selected_section_id) = app.selected_section_id.as_ref() {
                        if *selected_section_id == section.id {
                            Color32::BLUE.gamma_multiply(0.8)
                        } else {
                            Color32::BLACK.gamma_multiply(0.8)
                        }
                    } else {
                        Color32::BLACK.gamma_multiply(0.8)
                    },
                    symbol_background: if let Some(selected_section_id) = app.selected_section_id.as_ref() {
                        if *selected_section_id == section.id {
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
    }

    places
}

fn get_airport_places(app: &mut GsxmanApp) -> Vec<GsxPlace> {
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

pub fn zoom_map_to_position(app: &mut GsxmanApp, position: Position, zoom_level: u32) {
    app.map_memory.center_at(position);

    // Since we don't have any fine control of the zoom level outside of the library we use this hack
    // First zoom in all the way (until zoom_in() returns Err(InvalidZoom)), then zoom out as far as we need
    while app.map_memory.zoom_in().is_ok() {}
    for _ in 0..zoom_level {
        match app.map_memory.zoom_out() {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}
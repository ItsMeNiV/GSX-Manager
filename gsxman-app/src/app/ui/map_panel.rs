use egui::{Color32, Ui, Vec2};
use itertools::Itertools;
use walkers::{
    extras::{Place, Places, Style},
    Map, Position, Projector,
};

use crate::app::ui::UIState;
use crate::{app::GsxmanApp, core::GsxPlace};

use super::{filter_profile_details, filter_profiles};

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
            let _ = app.map_memory.zoom_in();
        } else if scroll_delta.y < 0.0 {
            app.map_memory.center_at(projected_pointer_pos);
            let _ = app.map_memory.zoom_out();
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

    if app.click_watcher.has_clicked {
        if let Some(clicked_label) = &app.click_watcher.clicked_label {
            let filter_text = app.filter_text.clone();
            match app.ui_state {
                UIState::Overview => {
                    for (row_index, (_, profile)) in app
                        .installed_gsx_profiles
                        .iter()
                        .sorted_by(|a, b| Ord::cmp(&a.1.airport.icao, &b.1.airport.icao))
                        .filter(|&(_, profile)| filter_profiles(&filter_text, profile)).enumerate()
                    {
                        if *clicked_label == profile.airport.icao {
                            app.selected_profile_id = Some(profile.id);
                            app.scroll_to_row = Some(row_index);
                            break;
                        }
                    }
                }
                UIState::Details => {
                    for (_, profile) in app.installed_gsx_profiles.iter() {
                        if let Some(profile_data) = profile.profile_data.as_ref() {
                            for (row_index, section) in profile_data
                                .sections
                                .iter()
                                .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
                                .filter(|&section| filter_profile_details(&filter_text, section)).enumerate()
                            {
                                if *clicked_label == section.name {
                                    app.selected_section_id = Some(section.id);
                                    app.scroll_to_row = Some(row_index);
                                    break;
                                }
                            }
                        }
                    }
                }
                UIState::SectionDetails => (),
                UIState::Notes => ()
            }
        } else {
            match app.ui_state {
                UIState::Overview => app.selected_profile_id = None,
                UIState::Details => app.selected_section_id = None,
                UIState::SectionDetails => (),
                UIState::Notes => ()
            }
        }

        app.click_watcher.has_clicked = false;
    }
}

fn get_places_to_display(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    match app.ui_state {
        UIState::Overview => get_airport_places(app),
        UIState::Details => get_airport_detail_places(app),
        UIState::SectionDetails => get_section_detail_places(app),
        UIState::Notes => get_airport_detail_places(app)
    }
}

fn get_section_detail_places(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    let filter_text = app.filter_text.clone().to_lowercase();
    let filter_text_str = filter_text.as_str();
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
                if pushback_label_left.to_lowercase().contains(filter_text_str) {
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
        }
        if let Some(pushback_position_right) = &selected_section.pushback_position_right {
            if let Some(pushback_label_right) = &selected_section.pushback_label_right {
                if pushback_label_right
                    .to_lowercase()
                    .contains(filter_text_str)
                {
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
    }
    places
}

fn get_airport_detail_places(app: &mut GsxmanApp) -> Vec<GsxPlace> {
    let filter_text = app.filter_text.clone();
    let mut places: Vec<GsxPlace> = vec![];
    if let Some(profile_data) = &app.get_selected_profile().as_ref().unwrap().profile_data {
        for section in profile_data
            .sections
            .iter()
            .filter(|&section| filter_profile_details(&filter_text, section))
        {
            places.push(GsxPlace(Place {
                label: section.name.to_owned(),
                position: Position::from_lat_lon(section.position.lat(), section.position.lon()),
                symbol: '🖈',
                style: Style {
                    label_background: if let Some(selected_section_id) =
                        app.selected_section_id.as_ref()
                    {
                        if *selected_section_id == section.id {
                            Color32::BLUE.gamma_multiply(0.8)
                        } else {
                            Color32::BLACK.gamma_multiply(0.8)
                        }
                    } else {
                        Color32::BLACK.gamma_multiply(0.8)
                    },
                    symbol_background: if let Some(selected_section_id) =
                        app.selected_section_id.as_ref()
                    {
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
    let filter_text = app.filter_text.clone();
    let mut places: Vec<GsxPlace> = Vec::new();
    for (_, profile) in app
        .installed_gsx_profiles
        .iter()
        .filter(|&(_, profile)| filter_profiles(&filter_text, profile))
    {
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
        let _ = app.map_memory.zoom_out();
    }
}

use egui::{
    text::{LayoutJob, LayoutSection, TextWrapping},
    Color32, FontId, Pos2, Rect, Rounding, TextFormat, Vec2,
};
use walkers::{extras::Place, Plugin, Position};

use crate::core::ProfileFile;

pub struct ClickWatcher {
    pub places: Option<Vec<Place>>,
    pub clicked_label: Option<String>,
    pub has_clicked: bool,
}

impl Plugin for &mut ClickWatcher {
    fn run(
        &mut self,
        response: &egui::Response,
        _painter: egui::Painter,
        projector: &walkers::Projector,
    ) {
        let click_position =
            if !response.changed() && response.clicked_by(egui::PointerButton::Primary) {
                response.interact_pointer_pos()
            } else {
                None
            };

        if let Some(click_pos) = click_position {
            self.has_clicked = true;
            self.clicked_label = None;

            if let Some(places) = &self.places {
                places.iter().for_each(|p| {
                    let place_position = projector.project(p.position);
                    let offset: f32 = 10.0;
                    if click_pos.x > (place_position.x - offset)
                        && click_pos.x < (place_position.x + offset)
                        && click_pos.y > (place_position.y - offset)
                        && click_pos.y < (place_position.y + offset)
                    {
                        self.clicked_label = Some(p.label.to_owned());
                    }
                });
            }
        }
    }
}

pub struct NoteDrawer {
    selected_profile: Option<ProfileFile>,
}

impl Plugin for NoteDrawer {
    fn run(
        &mut self,
        _response: &egui::Response,
        painter: egui::Painter,
        projector: &walkers::Projector,
    ) {
        if let Some(selected_profile) = &self.selected_profile {
            if selected_profile.notes.is_empty() {
                return;
            }
            let notes_to_display = (&selected_profile.notes).clone();

            let profile_location = selected_profile.airport.location;
            let profile_map_position = projector.project(Position::from_lat_lon(
                profile_location.latitude(),
                profile_location.longitude(),
            ));
            let rect_size = Vec2::new(100.0, 100.0);
            let rect_y_offset = 15.0;
            let rect_position = Pos2::new(
                profile_map_position.x,
                profile_map_position.y - (rect_y_offset + (rect_size.y / 2.0)),
            );
            let rect = Rect::from_center_size(rect_position, rect_size);
            painter.rect_filled(rect, Rounding::from(5.0), Color32::BLACK);

            let text_margin = 2.0;
            let text_position = Pos2::new(
                rect_position.x - (rect_size.x / 2.0) + text_margin,
                rect_position.y - (rect_size.y / 2.0) + text_margin,
            );
            let layout_job = LayoutJob {
                sections: vec![LayoutSection {
                    leading_space: 0.0,
                    byte_range: 0..notes_to_display.len(),
                    format: TextFormat::simple(FontId::monospace(10.0), Color32::WHITE),
                }],
                text: notes_to_display,
                wrap: TextWrapping {
                    max_width: rect_size.x - (2.0 * text_margin),
                    max_rows: 7,
                    ..Default::default()
                },
                ..Default::default()
            };
            let galley = painter.layout_job(layout_job);
            painter.galley(text_position, galley, Color32::WHITE);
        }
    }
}

impl NoteDrawer {
    pub fn new(selected_profile: Option<ProfileFile>) -> Self {
        Self { selected_profile }
    }
}

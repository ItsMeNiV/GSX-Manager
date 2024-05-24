use walkers::{extras::Place, Plugin};

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
                    let airport_position = projector.project(p.position);
                    let offset = 10.0;
                    if click_pos.x > (airport_position.x - offset)
                        && click_pos.x < (airport_position.x + offset)
                        && click_pos.y > (airport_position.y - offset)
                        && click_pos.y < (airport_position.y + offset)
                    {
                        self.clicked_label = Some(p.label.to_owned());
                    }
                });
            }
        }
    }
}

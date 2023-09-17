use crate::{data_types::Complexity, model::Model, view::View};

pub struct Controller<'a> {
    view: &'a View,
    model: &'a mut Model,
}

impl<'a> Controller<'a> {
    pub fn new(view_inst: &'a View, model_inst: &'a mut Model) -> Self {
        let view = view_inst;
        let model = model_inst;

        Self { view, model }
    }

    pub fn start(&mut self) {
        let difficulty = self
            .view
            .choose_difficulty()
            .expect("Failed to choose difficulty");

        let (width, height, mine_count) = match difficulty {
            Complexity::Beginner => (9, 9, 10),
            Complexity::Intermediate => (16, 16, 40),
            Complexity::Advanced => (24, 24, 99),
        };

        self.model.create_game_area(width, height, mine_count);

        loop {
            let (x, y, action) = match self.view.choose_coord_action(self.model.get_game_area()) {
                Ok(coords) => coords,
                Err(error) => {
                    println!("Error: {}", error);
                    continue;
                }
            };

            self.model.set_cell(x, y, action);
            self.view.print_area(self.model.get_game_area());
            // break;
        }
    }
}

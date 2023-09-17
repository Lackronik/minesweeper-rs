mod data_types;

use model::Model;
mod model;
use controller::Controller;
mod controller;
use view::View;
mod view;

fn main() {
    let view = View::new();
    let mut model = Model::new();

    let mut controller = Controller::new(&view, &mut model);

    controller.start();
}

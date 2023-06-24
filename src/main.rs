mod data_types;
mod controller;
mod model;

//UNIT TESTS FOR VERIFICATION
fn unit_test(input_str: &str) {
    match controller::verify_input(input_str) {
        Ok(coords) => println!("X:{} | Y:{}", coords.x, coords.y),
        Err(err) => println!("Error: {}", err),
    };
}

fn main() {
    unit_test("10 5 f");
    unit_test("k 5 o");
    unit_test("5 f u");
    unit_test("12 16 k");


}


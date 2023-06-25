#[cfg(test)]
mod tests {
    use super::*;

    fn verify_input(input_str: &str) {
        match controller::verify_input(input_str) {
            Ok(coords) => println!("X:{} | Y:{}", coords.x, coords.y),
            Err(err) => println!("Error: {}", err),
        };
    }

    #[test]
    fn test_verify_input() {
        verify_input("10 5 f");
            unit_test("k 5 o");
            unit_test("5 f u");
            unit_test("12 16 k");
            unit_test("12 15 f");
    }
}

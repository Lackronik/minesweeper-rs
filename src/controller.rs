use super::data_types;

pub struct Controller {}

impl Controller {}

pub fn verify_input(input_line: &str) -> Result<data_types::Coord, String> {
    let vec_coord: Vec<&str> = input_line.split_whitespace().collect();

    let x = vec_coord[0].parse().map_err(|_| "Invalid first uint")?;
    let y = vec_coord[1].parse().map_err(|_| "Invalid second uint")?;

    let action = match vec_coord[2] {
        "f" => data_types::Action::FLAGCELL,
        "u" => data_types::Action::UNFLAGGCELL,
        "o" => data_types::Action::CHECKCELL,
        _ => return Err("Invalid key for cell action".to_string()),
    };

    let result = data_types::Coord { x, y, action };

    Ok(result)
}

use super::data_types;

pub fn verify_input(input_line: &str) -> Result<data_types::Coord, String> {
    let vec_coord  = input_line.split_whitespace().collect::<Vec<_>>();
    let mut result: data_types::Coord = data_types::Coord { x: 0, y: 0, action: data_types::Action::CHECKCELL };
    match vec_coord[0].parse::<u16>() {
        Ok(i) => result.x = i,
        Err(..) => return Err("first uint not correct".to_string())
    };
    match vec_coord[1].parse::<u16>() {
        Ok(i) => result.y = i,
        Err(..) => return Err("second uint not correct".to_string())
    };
    match vec_coord[2] {
        "f" => result.action = data_types::Action::FLAGCELL,
        "u" => result.action = data_types::Action::UNFLAGGCELL,
        "o" => result.action = data_types::Action::CHECKCELL,
        _   => return Err("Not correct key for cell action".to_string())
    };
    return Ok(result)
}


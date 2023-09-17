use std::io::{self};

use crate::data_types::Complexity;

use super::data_types;

pub struct View {}

impl View {
    pub fn new() -> Self {
        View {}
    }

    pub fn print_area(&self, area_to_print: &Vec<Vec<data_types::CellStatus>>) {
        for row in area_to_print {
            for cell in row {
                print!("{}", cell.as_str());
                if let data_types::CellStatus::OPENED(val) = cell {
                    if *val > 1 {
                        print!("");
                    }
                }
            }
            println!();
        }
    }

    pub fn choose_difficulty(&self) -> Result<Complexity, String> {
        loop {
            println!("Choose the difficulty level:");
            println!("1. Beginner - 9x9 & 10 💣");
            println!("2. Intermediate - 16x16 & 40 💣");
            println!("3. Advanced - 24x24 & 99 💣");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse::<usize>() {
                Ok(choice) => match choice {
                    1 => return Ok(Complexity::Beginner),
                    2 => return Ok(Complexity::Intermediate),
                    3 => return Ok(Complexity::Advanced),
                    _ => println!(
                        "❗Invalid choice. Please choose a valid difficulty level (1-3).❗"
                    ),
                },
                Err(_) => {
                    println!("❗Invalid input. Please enter a number (1-3).❗");
                }
            }
        }
    }

    pub fn choose_coord_action(
        &self,
        area_to_print: &Vec<Vec<data_types::CellStatus>>,
    ) -> Result<(u16, u16, data_types::Action), String> {
        loop {
            println!("Choose coordinate and action (e.g., 5 5 o):");
            self.print_area(area_to_print);

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input_parts: Vec<&str> = input.split_whitespace().collect();
                    if input_parts.len() != 3 {
                        println!("Invalid input format. Please try again.");
                        continue;
                    }

                    let x: u16 = match input_parts[0].parse() {
                        Ok(value) => value,
                        Err(_) => {
                            println!("Invalid x coordinate. Please try again.");
                            continue;
                        }
                    };

                    let y: u16 = match input_parts[1].parse() {
                        Ok(value) => value,
                        Err(_) => {
                            println!("Invalid y coordinate. Please try again.");
                            continue;
                        }
                    };

                    let action = match input_parts[2] {
                        "o" => data_types::Action::CHECKCELL,
                        "f" => data_types::Action::FLAGCELL,
                        "u" => data_types::Action::UNFLAGCELL,
                        _ => {
                            println!("Invalid action. Please try again.");
                            continue;
                        }
                    };

                    return Ok((x, y, action));
                }
                Err(error) => {
                    return Err(error.to_string());
                }
            }
        }
    }
}

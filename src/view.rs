use super::data_types;

pub struct View {}

impl View {
    pub fn print_area(area_to_print: &Vec<Vec<data_types::CellStatus>>) -> Result<(), ()> {
        for i in 0..area_to_print.len() {
            for j in 0..area_to_print[i].len() {
                print!("{}", area_to_print[i][j].as_str());
            }
            println!();
        }
        Ok(())
    }
}
// mine_area.len(), mine_area[0].len()

use super::data_types;

pub struct View {
  
}

impl View {
  pub fn print_area(area_to_print& Vec<Vec<data_types::CellStatus>>)  -> Result {
    for i in 0..area_to_print.len() {
    for j in 0..area_to_print[i].len() {
      print!("{}", mine_area[i as usize][j as usize].as_str());
    }
    println!("");
  }
  }
}
mine_area.len(), mine_area[0].len()

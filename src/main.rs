use solver::start_resolution;

mod solver;

fn main() {
    match start_resolution(std::env::args()) {
        Ok(sudoku) =>
            println!(
                "The solution is: \n\t{}", 
                sudoku.0.iter().flat_map(|column| column.map(|n| n.to_string())).collect::<String>()
            ),
        Err(message) => eprintln!("[error] {}", message)
    }
}

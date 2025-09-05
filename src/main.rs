use solver::start_resolution;

mod solver;

fn main() {
    let timer = std::time::Instant::now();

    match start_resolution(std::env::args()) {
        Ok(sudoku) => {
            let duration = timer.elapsed();
            println!(
                "Process finished in {}ms \nThe solution is: \n\t{}",
                duration.as_millis(),
                sudoku.0.iter().flat_map(|column| column.map(|n| n.to_string())).collect::<String>()
            )
        },
        Err(message) => eprintln!("[error] {}", message)
    }
}

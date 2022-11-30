use crate::utils::read_input;

mod day_01;
mod utils;

fn main() {
    let days = [day_01::main];

    for (i, solve_day) in days.into_iter().enumerate() {
        let day = i + 1;

        println!("\n\nDay {}:", day);

        solve_day(read_input(day));
    }
}

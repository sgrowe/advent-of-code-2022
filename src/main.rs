use crate::utils::read_input;

mod day_01;
mod day_02;
mod day_03;
mod groups_of;
mod utils;

fn main() {
    let days = [day_01::main, day_02::main, day_03::main];

    for (i, solve_day) in days.into_iter().enumerate() {
        let day = i + 1;

        println!("\n\nDay {}:", day);

        solve_day(&read_input(day));
    }
}

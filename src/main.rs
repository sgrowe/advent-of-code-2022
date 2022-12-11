use crate::utils::read_input;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod groups_of;
mod solution;
mod utils;

fn main() {
    let days = [
        day_01::main,
        day_02::main,
        day_03::main,
        day_04::main,
        day_05::main,
        day_06::main,
        day_07::main,
        day_08::main,
        day_09::main,
        day_10::main,
    ];

    for (i, solve_day) in days.into_iter().enumerate() {
        let day = i + 1;

        println!("\n\n-- Day {} --", day);

        solve_day(&read_input(day));
    }
}

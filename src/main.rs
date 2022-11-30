mod day_01;

fn main() {
    let days = [day_01::main];

    for (i, day) in days.into_iter().enumerate() {
        println!("\n\nDay {}:", i + 1);

        day();
    }
}

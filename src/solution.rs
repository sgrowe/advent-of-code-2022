use std::fmt::Display;

pub struct Solution<A, B> {
    pub part_one: A,
    pub part_two: B,
}

impl<A, B> Solution<A, B>
where
    A: Display,
    B: Display,
{
    pub fn print(&self) {
        println!("Part one: {}", self.part_one);
        println!("Part two: {}", self.part_two);
    }
}

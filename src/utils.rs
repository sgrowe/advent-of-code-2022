use std::{fmt::Debug, fs::read_to_string, str::FromStr};

pub fn read_input(day: usize) -> String {
    read_to_string(format!("inputs/day_{:02}.txt", day)).unwrap()
}

pub fn parse_lines<'a, F: FromStr>(s: &'a str) -> impl Iterator<Item = F> + 'a
where
    <F as FromStr>::Err: Debug,
{
    s.trim().lines().map(|l| l.parse::<F>().unwrap())
}

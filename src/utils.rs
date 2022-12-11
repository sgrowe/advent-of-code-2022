use std::{fmt::Debug, fs::read_to_string, str::FromStr};

pub fn read_input(day: usize) -> String {
    read_to_string(format!("inputs/day_{:02}.txt", day)).unwrap()
}

pub fn parse_as<F: FromStr>(s: &str) -> F
where
    <F as FromStr>::Err: Debug,
{
    s.parse::<F>().unwrap()
}

use std::{
    fmt::{Display, Write},
    num::ParseIntError,
    str::{Chars, FromStr},
};

use crate::{solution::Solution, utils::parse_as};

pub fn main(input: &str) {
    solution(input).print();
}

fn solution(input: &str) -> Solution<Stacks, Stacks> {
    let mut lines = input.lines();

    let mut stacks_a = parse_stacks(&mut lines);
    let mut stacks_b = stacks_a;

    // Skip blank line
    lines.next().unwrap();

    for instruction in lines.map(parse_as) {
        stacks_a.update_v1(instruction);

        stacks_b.update_v2(instruction);
    }

    Solution {
        part_one: stacks_a,
        part_two: stacks_b,
    }
}

fn parse_stacks<'a, I>(lines: &mut I) -> Stacks
where
    I: Iterator<Item = &'a str>,
{
    let mut stacks = Stacks::default();

    for line in lines {
        if line.starts_with(" 1 ") {
            return stacks;
        }

        for (i, item) in StackLineParser::new(line).enumerate() {
            if let Some(c) = item {
                stacks.0[i].add_to_bottom(c);
            }
        }
    }

    panic!("Did not reach end of stacks")
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    num: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.strip_prefix("move ").unwrap().split(' ');

        let num = words.next().unwrap().parse()?;

        // from
        words.next().unwrap();

        let from = words.next().unwrap().parse()?;

        // to
        words.next().unwrap();

        let to = words.next().unwrap().parse()?;

        Ok(Self { num, from, to })
    }
}

struct StackLineParser<'a> {
    chars: Chars<'a>,
}

impl<'a> StackLineParser<'a> {
    pub fn new(l: &'a str) -> Self {
        Self { chars: l.chars() }
    }
}

impl<'a> Iterator for StackLineParser<'a> {
    type Item = Option<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()?; // [ char

        let c = self.chars.next()?;

        let _ = self.chars.next(); // ] char
        let _ = self.chars.next(); // possible space separator

        let item = if c == ' ' { None } else { Some(c) };

        Some(item)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Stacks(pub [Stack; 9]);

impl Stacks {
    fn update_v1(&mut self, Instruction { num, from, to }: Instruction) {
        for _ in 0..num {
            let c = self.0[from - 1].pop();

            self.0[to - 1].add_to_top(c);
        }
    }

    fn update_v2(&mut self, Instruction { num, from, to }: Instruction) {
        let from_stack = &self.0[from - 1];
        let from_len = from_stack.len;

        let mut items = [' '; STACK_MAX_SIZE];
        (&mut items[..num]).copy_from_slice(&from_stack.items[from_len - num..from_len]);

        let to_stack = &mut self.0[to - 1];
        let to_len = to_stack.len;
        (&mut to_stack.items[to_len..to_len + num]).copy_from_slice(&items[..num]);

        to_stack.len += num;
        self.0[from - 1].len -= num
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stack in self.0 {
            if stack.len == 0 {
                break;
            }

            f.write_char(stack.items[stack.len - 1])?;
        }

        Ok(())
    }
}

const STACK_MAX_SIZE: usize = 40;

#[derive(Debug, Clone, Copy)]
struct Stack {
    len: usize,
    items: [char; STACK_MAX_SIZE],
}

impl Default for Stack {
    fn default() -> Self {
        Stack {
            len: 0,
            items: [' '; STACK_MAX_SIZE],
        }
    }
}

impl Stack {
    pub fn pop(&mut self) -> char {
        let c = self.items[self.len - 1];
        self.len -= 1;
        c
    }

    pub fn add_to_top(&mut self, i: char) {
        self.items[self.len] = i;
        self.len += 1;
    }

    pub fn add_to_bottom(&mut self, c: char) {
        let mut i = self.len;

        while i > 0 {
            self.items[i] = self.items[i - 1];
            i -= 1;
        }

        self.items[0] = c;

        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn part_one() {
        assert_eq!(
            format!("{}", solution(SAMPLE.trim_matches('\n')).part_one),
            "CMZ"
        );

        assert_eq!(
            format!("{}", solution(&read_input(5)).part_one),
            "GFTNRBZPF"
        );
    }

    #[test]
    fn part_two() {
        assert_eq!(
            format!("{}", solution(SAMPLE.trim_matches('\n')).part_two),
            "MCD"
        );

        assert_eq!(
            format!("{}", solution(&read_input(5)).part_two),
            "VRQWPDSGP"
        );
    }
}

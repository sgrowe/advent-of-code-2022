use std::fmt::{Display, Formatter, Write};

use crate::solution::Solution;

pub fn main(input: &str) {
    solution(input).print();
}

fn solution(input: &str) -> Solution<isize, CRT> {
    let mut cpu = CPU::new();

    for instr in input.lines().map(Instr::from) {
        match instr {
            Instr::NoOp => {
                cpu.next_cycle();
            }
            Instr::Add(x) => {
                cpu.next_cycle();
                cpu.next_cycle();

                cpu.add_to_register(x);
            }
        }
    }

    Solution {
        part_one: cpu.sum_special_signals,
        part_two: cpu.crt,
    }
}

#[derive(Debug, Clone, Copy)]
struct CPU {
    register: isize,
    cycle_no: isize,
    sum_special_signals: isize,
    crt: CRT,
}

impl CPU {
    fn new() -> Self {
        Self {
            register: 1,
            cycle_no: 0,
            sum_special_signals: 0,
            crt: CRT::new(),
        }
    }

    fn next_cycle(&mut self) {
        self.cycle_no += 1;

        self.crt.next_cycle(self.register);

        if (self.cycle_no - 20) % 40 == 0 {
            self.sum_special_signals += self.cycle_no * self.register;
        }
    }

    fn add_to_register(&mut self, x: isize) {
        self.register += x
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    NoOp,
    Add(isize),
}

impl From<&str> for Instr {
    fn from(s: &str) -> Self {
        if s == "noop" {
            return Instr::NoOp;
        }

        if let Some(x) = s.strip_prefix("addx ") {
            return Instr::Add(x.parse().unwrap());
        }

        panic!("Unknown instruction")
    }
}

#[derive(Debug, Clone, Copy)]
struct CRT {
    pixels: [[bool; 40]; 6],
    pos: usize,
}

impl CRT {
    fn new() -> Self {
        Self {
            pixels: [[false; 40]; 6],
            pos: 0,
        }
    }

    fn next_cycle(&mut self, register: isize) {
        let row = self.pos / 40;
        let col = self.pos % 40;

        self.pixels[row][col] = col.abs_diff(register as usize) <= 1;

        self.pos += 1;
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_char('\n')?;

        for row in self.pixels {
            for pixel in row {
                f.write_char(if pixel { '#' } else { '.' })?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
    ";

    #[test]
    fn part_one() {
        assert_eq!(solution(SAMPLE.trim()).part_one, 13140);

        assert_eq!(solution(&read_input(10)).part_one, 14040);
    }

    #[test]
    fn part_two() {
        let expected = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        similar_asserts::assert_eq!(format!("{}", solution(SAMPLE.trim()).part_two), expected);

        let expected_two = "
####..##...##....##.####...##.####.#....
...#.#..#.#..#....#....#....#.#....#....
..#..#....#.......#...#.....#.###..#....
.#...#.##.#.......#..#......#.#....#....
.....#..#.#..#.#..#.#....#..#.#....#....
####..###..##...##..####..##..#....####.
";

        similar_asserts::assert_eq!(
            format!("{}", solution(&read_input(10)).part_two),
            expected_two
        );
    }
}

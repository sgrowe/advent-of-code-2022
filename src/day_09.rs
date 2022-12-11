use std::collections::HashSet;

use crate::solution::Solution;

pub fn main(input: &str) {
    solution(input).print();
}

type Coord = (isize, isize);

fn solution(input: &str) -> Solution<usize, usize> {
    // Part one
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut has_visited = HashSet::new();

    // Part two
    let mut many_knots = [(0, 0); 10];
    let mut has_visited_v2 = HashSet::new();

    for (dir, steps) in input.lines().map(parse_line) {
        for _ in 0..steps {
            // Part one
            update_head(&mut head, dir);

            update_tail(head, &mut tail);

            has_visited.insert(tail);

            // Part two
            update_head(&mut many_knots[0], dir);

            for i in 1..many_knots.len() {
                update_tail(many_knots[i - 1], &mut many_knots[i]);
            }

            has_visited_v2.insert(many_knots[9]);
        }
    }

    Solution {
        part_one: has_visited.len(),
        part_two: has_visited_v2.len(),
    }
}

fn update_head(head: &mut Coord, dir: Dir) {
    match dir {
        Dir::Up => head.1 += 1,
        Dir::Down => head.1 -= 1,
        Dir::Left => head.0 -= 1,
        Dir::Right => head.0 += 1,
    }
}

fn update_tail(head: Coord, tail: &mut Coord) {
    let is_touching = head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1;

    if is_touching {
        return;
    }

    if head.0 > tail.0 {
        tail.0 += 1;
    } else if head.0 < tail.0 {
        tail.0 -= 1;
    }

    if head.1 > tail.1 {
        tail.1 += 1;
    } else if head.1 < tail.1 {
        tail.1 -= 1;
    }
}

fn parse_line(line: &str) -> (Dir, usize) {
    let (dir, steps) = line.split_once(' ').unwrap();

    (dir.into(), steps.parse().unwrap())
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "U" => Dir::Up,
            "L" => Dir::Left,
            "R" => Dir::Right,
            "D" => Dir::Down,
            _ => panic!("Unexpected direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn part_one() {
        assert_eq!(solution(SAMPLE.trim()).part_one, 13);

        assert_eq!(solution(&read_input(9)).part_one, 6563);
    }

    const SAMPLE_TWO: &str = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn part_two() {
        assert_eq!(solution(SAMPLE.trim()).part_two, 1);

        assert_eq!(solution(SAMPLE_TWO.trim()).part_two, 36);

        assert_eq!(solution(&read_input(9)).part_two, 2653);
    }
}

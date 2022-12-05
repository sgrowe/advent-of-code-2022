use std::{ops::RangeInclusive, str::FromStr};

use crate::{solution::Solution, utils::parse_as};

pub fn main(input: &str) {
    overlapping_assignments(input).print();
}

fn overlapping_assignments(input: &str) -> Solution<usize, usize> {
    let mut fully_contained_count = 0;
    let mut any_overlap_count = 0;

    for l in input.lines() {
        let pair = parse_as::<AssignmentPair>(l);

        if pair.one_fully_contains_the_other() {
            fully_contained_count += 1;
        }

        if pair.has_any_overlap() {
            any_overlap_count += 1;
        }
    }

    Solution {
        part_one: fully_contained_count,
        part_two: any_overlap_count,
    }
}

struct AssignmentPair {
    a: RangeInclusive<usize>,
    b: RangeInclusive<usize>,
}

impl AssignmentPair {
    fn one_fully_contains_the_other(&self) -> bool {
        let AssignmentPair { a, b } = self;

        (a.contains(b.start()) && a.contains(b.end()))
            || (b.contains(a.start()) && b.contains(a.end()))
    }

    fn has_any_overlap(&self) -> bool {
        let AssignmentPair { a, b } = self;

        a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
    }
}

impl FromStr for AssignmentPair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(',').unwrap();

        Ok(AssignmentPair {
            a: parse_range(a),
            b: parse_range(b),
        })
    }
}

fn parse_range(s: &str) -> RangeInclusive<usize> {
    let (start, end) = s.split_once('-').unwrap();

    RangeInclusive::new(parse_as(start), parse_as(end))
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn part_one() {
        assert_eq!(overlapping_assignments(SAMPLE.trim()).part_one, 2);

        assert_eq!(overlapping_assignments(&read_input(4)).part_one, 651);
    }

    #[test]
    fn part_two() {
        assert_eq!(overlapping_assignments(SAMPLE.trim()).part_two, 4);

        assert_eq!(overlapping_assignments(&read_input(4)).part_two, 956);
    }
}

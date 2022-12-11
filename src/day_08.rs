use crate::solution::Solution;
use std::cmp::max;

pub fn main(input: &str) {
    solution(input).print();
}

fn solution(input: &str) -> Solution<usize, usize> {
    let map = Map::new(input.trim().as_bytes());

    map.solution()
}

type Coord = (usize, usize);

struct Map<'a> {
    width: usize,
    height: usize,
    map: &'a [u8],
}

impl<'a> Map<'a> {
    fn new(map: &'a [u8]) -> Self {
        let width = map.iter().copied().take_while(|&c| c != b'\n').count() + 1;

        let height = (map.len() / width) + 1;

        Self { width, height, map }
    }

    fn trees_around_edge(&self) -> usize {
        2 * self.height + (2 * (self.width - 3))
    }

    fn solution(&self) -> Solution<usize, usize> {
        let mut trees_visible = self.trees_around_edge();
        let mut top_score = 0;

        for x in 1..(self.width - 2) {
            for y in 1..(self.height - 1) {
                top_score = max(top_score, self.scenic_score(x, y));

                if self.is_tree_visible(x, y) {
                    trees_visible += 1;
                }
            }
        }

        Solution {
            part_one: trees_visible,
            part_two: top_score,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.map[(y * self.width) + x]
    }

    fn all_dirs_from_point(
        &self,
        x: usize,
        y: usize,
    ) -> (
        impl Iterator<Item = Coord>,
        impl Iterator<Item = Coord>,
        impl Iterator<Item = Coord>,
        impl Iterator<Item = Coord>,
    ) {
        let left = (0..x).map(move |x| (x, y)).rev();
        let right = (x + 1..self.width).map(move |x| (x, y));
        let top = (0..y).map(move |y| (x, y)).rev();
        let bottom = (y + 1..self.height).map(move |y| (x, y));

        (left, right, top, bottom)
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.get(x, y);

        let (left, right, top, bottom) = self.all_dirs_from_point(x, y);

        self.is_tallest_amongst(tree_height, left)
            || self.is_tallest_amongst(tree_height, right)
            || self.is_tallest_amongst(tree_height, top)
            || self.is_tallest_amongst(tree_height, bottom)
    }

    fn is_tallest_amongst<I>(&self, tree_height: u8, coords: I) -> bool
    where
        I: Iterator<Item = Coord>,
    {
        for (x, y) in coords {
            if self.get(x, y) >= tree_height {
                return false;
            }
        }

        true
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let tree_height = self.get(x, y);

        let (left, right, top, bottom) = self.all_dirs_from_point(x, y);

        self.trees_in_sight(tree_height, left)
            * self.trees_in_sight(tree_height, right)
            * self.trees_in_sight(tree_height, top)
            * self.trees_in_sight(tree_height, bottom)
    }

    fn trees_in_sight<I>(&self, tree_height: u8, coords: I) -> usize
    where
        I: Iterator<Item = Coord>,
    {
        let mut count = 0;

        for (x, y) in coords {
            count += 1;

            if self.get(x, y) >= tree_height {
                break;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
30373
25512
65332
33549
35390
";

    #[test]
    fn part_one() {
        assert_eq!(solution(SAMPLE.trim()).part_one, 21);

        assert_eq!(solution(&read_input(8)).part_one, 1676);
    }

    #[test]
    fn part_two() {
        assert_eq!(solution(SAMPLE.trim()).part_two, 8);

        assert_eq!(solution(&read_input(8)).part_two, 313200);
    }
}

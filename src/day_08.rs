use crate::solution::Solution;

pub fn main(input: &str) {
    solution(input).print();
}

fn solution(input: &str) -> Solution<usize, usize> {
    let map = Map::new(input.trim().as_bytes());

    Solution {
        part_one: map.trees_visible(),
        part_two: map.top_scenic_score(),
    }
}

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

    fn trees_visible(&self) -> usize {
        let mut trees_visible = self.trees_around_edge();

        for i in 1..(self.width - 2) {
            for j in 1..(self.height - 1) {
                if self.is_tree_visible(i, j) {
                    trees_visible += 1;
                }
            }
        }

        trees_visible
    }

    fn top_scenic_score(&self) -> usize {
        let mut top_score = 0;

        for i in 1..(self.width - 2) {
            for j in 1..(self.height - 1) {
                top_score = std::cmp::max(top_score, self.scenic_score(i, j));
            }
        }

        top_score
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.map[(y * self.width) + x]
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree_height = self.get(x, y);

        let left = (0..x).map(|x| (x, y));
        let right = (x + 1..self.width).map(|x| (x, y));
        let top = (0..y).map(|y| (x, y));
        let bottom = (y + 1..self.height).map(|y| (x, y));

        self.is_tallest_amongst(tree_height, left)
            || self.is_tallest_amongst(tree_height, right)
            || self.is_tallest_amongst(tree_height, top)
            || self.is_tallest_amongst(tree_height, bottom)
    }

    fn is_tallest_amongst<I>(&self, tree_height: u8, coords: I) -> bool
    where
        I: Iterator<Item = (usize, usize)>,
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

        let left = (0..x).map(|x| (x, y)).rev();
        let right = (x + 1..self.width).map(|x| (x, y));
        let top = (0..y).map(|y| (x, y)).rev();
        let bottom = (y + 1..self.height).map(|y| (x, y));

        self.trees_in_sight(tree_height, left)
            * self.trees_in_sight(tree_height, right)
            * self.trees_in_sight(tree_height, top)
            * self.trees_in_sight(tree_height, bottom)
    }

    fn trees_in_sight<I>(&self, tree_height: u8, coords: I) -> usize
    where
        I: Iterator<Item = (usize, usize)>,
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

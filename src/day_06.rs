use crate::solution::Solution;

pub fn main(input: &str) {
    solution(input).print();
}

fn solution(input: &str) -> Solution<usize, usize> {
    let data = input.as_bytes();

    let mut i = 0;

    loop {
        let min_step = min_next_marker_step(&data[i..i + 4]);

        i += min_step;

        if min_step == 0 {
            break;
        }
    }

    let part_one = i + 4;

    // Only once we’ve found 4 distinct characters in a row can we find 14
    // distinct chars in a row
    loop {
        let min_step = min_next_marker_step(&data[i..i + 14]);

        i += min_step;

        if min_step == 0 {
            break;
        }
    }

    let part_two = i + 14;

    Solution { part_one, part_two }
}

fn min_next_marker_step(group: &[u8]) -> usize {
    let mut i = 0;

    while i < group.len() - 1 {
        let mut j = i + 1;

        while j < group.len() {
            if group[i] == group[j] {
                return i + 1;
            }

            j += 1;
        }

        i += 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;
    use test_case::test_case;

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn part_one(input: &str, expected: usize) {
        assert_eq!(solution(input).part_one, expected);
    }

    #[test]
    fn part_one_result() {
        assert_eq!(solution(&read_input(6)).part_one, 1198);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn part_two(input: &str, expected: usize) {
        assert_eq!(solution(input).part_two, expected);
    }

    #[test]
    fn part_two_result() {
        assert_eq!(solution(&read_input(6)).part_two, 3120);
    }
}

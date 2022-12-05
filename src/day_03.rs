use crate::groups_of::groups_of;

pub fn main(input: &str) {
    println!("Part one: {}", sum_of_duplicate_priorities(input));
    println!("Part two: {}", sum_of_badge_priorities(input));
}

fn sum_of_duplicate_priorities(input: &str) -> usize {
    input.lines().map(|l| duplicate_item(l.as_bytes())).sum()
}

fn sum_of_badge_priorities(input: &str) -> usize {
    groups_of(input.lines().map(|l| l.as_bytes()))
        .map(groups_badge)
        .sum()
}

fn groups_badge(group: [&[u8]; 3]) -> usize {
    let in_a = item_priorities_in(group[0]);
    let in_b = item_priorities_in(group[1]);

    for &x in group[2] {
        let p = as_priority(x);

        if in_a[p - 1] && in_b[p - 1] {
            return p;
        }
    }

    panic!("Badge not found");
}

fn item_priorities_in(items: &[u8]) -> [bool; 52] {
    let mut already_had = [false; 52];

    for &x in items {
        already_had[as_priority(x) - 1] = true;
    }

    already_had
}

fn duplicate_item(rucksack: &[u8]) -> usize {
    let (a, b) = rucksack.split_at(rucksack.len() / 2);

    let already_had = item_priorities_in(a);

    for &x in b {
        let p = as_priority(x);

        if already_had[p - 1] {
            return p;
        }
    }

    panic!("No duplicate found");
}

fn as_priority(c: u8) -> usize {
    if c >= 97 {
        (c - 96) as usize
    } else {
        (c - 38) as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part_one() {
        assert_eq!(sum_of_duplicate_priorities(SAMPLE.trim()), 157);

        assert_eq!(sum_of_duplicate_priorities(&read_input(3)), 7980);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(sum_of_badge_priorities(SAMPLE.trim()), 70);

        assert_eq!(sum_of_badge_priorities(&read_input(3)), 2881);
    }
}

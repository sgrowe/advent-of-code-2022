pub fn main(input: &str) {
    let top_three = top_three_calorie_counts(input);

    println!("Part one: {}", top_three[0]);
    println!("Part two: {}", top_three.into_iter().sum::<usize>());
}

fn top_three_calorie_counts(input: &str) -> [usize; 3] {
    let mut top_three = [0; 3];

    for t in calories_per_elf(input) {
        if t <= top_three[2] {
            continue;
        }

        let mut i = 0;

        while i < top_three.len() {
            if t > top_three[i] {
                let mut j = top_three.len() - 1;

                while j > i {
                    top_three[j] = top_three[j - 1];
                    j -= 1;
                }

                top_three[i] = t;
                break;
            }

            i += 1;
        }
    }

    top_three
}

fn calories_per_elf<'a>(
    input: &'a str,
) -> CaloriesPerElf<impl Iterator<Item = Option<usize>> + 'a> {
    CaloriesPerElf {
        lines: input.lines().map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.parse::<usize>().unwrap())
            }
        }),
    }
}

struct CaloriesPerElf<I> {
    lines: I,
}

impl<I> Iterator for CaloriesPerElf<I>
where
    I: Iterator<Item = Option<usize>>,
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cals = self.lines.next()?.unwrap();

        while let Some(c) = self.lines.next().flatten() {
            cals += c;
        }

        Some(cals)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_input;

    use super::*;

    const SAMPLE: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_part_one() {
        assert_eq!(top_three_calorie_counts(SAMPLE.trim())[0], 24000);

        assert_eq!(top_three_calorie_counts(&read_input(1))[0], 69836);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            top_three_calorie_counts(SAMPLE.trim()),
            [24000, 11000, 10000]
        );

        assert_eq!(
            top_three_calorie_counts(&read_input(1))
                .into_iter()
                .sum::<usize>(),
            207968
        );
    }
}

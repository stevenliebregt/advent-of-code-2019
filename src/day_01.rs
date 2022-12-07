use crate::utils::LineIterator;
use aoc_runner_derive::aoc;

type Output = i32;

#[aoc(day1, part1)]
pub fn solve_part_1(input: &str) -> Output {
    let mut sum = 0;

    for line in LineIterator::from(input) {
        sum += (line.parse::<i32>().unwrap() / 3) - 2
    }

    sum
}

#[aoc(day1, part2)]
pub fn solve_part_2(input: &str) -> Output {
    let mut sum = 0;

    for line in LineIterator::from(input) {
        let mut value = line.parse::<i32>().unwrap();

        'dividing: loop {
            let fuel = (value / 3) - 2;

            if fuel <= 0 {
                break 'dividing;
            }

            sum += fuel;
            value = fuel;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 654;

        assert_eq!(expected, solve_part_1("1969".trim()));
    }

    #[test]
    fn test_part_2() {
        let expected = 966;

        assert_eq!(expected, solve_part_2("1969".trim()));
    }
}

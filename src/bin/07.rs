advent_of_code::solution!(7);

use itertools::{repeat_n, Itertools};
use rayon::prelude::*;

type Int = u64;

struct Calibration {
    target: Int,
    operands: Vec<Int>,
}

impl Calibration {
    fn from_str(input: &str) -> Self {
        let mut s = input.split(": ");
        Self {
            target: s
                .next()
                .expect("no target val")
                .parse::<Int>()
                .expect("couldn't parse target"),
            operands: s
                .next()
                .expect("no operands")
                .split(' ')
                .map(|o| {
                    o.parse::<Int>()
                        .unwrap_or_else(|_| panic!("couldn't parse {}", o))
                })
                .collect(),
        }
    }

    fn test_operators_produce_target<F: Fn(Int, Int) -> Int + Send + Sync>(
        &self,
        operators: &[F],
    ) -> bool {
        let placements = self.operands.len() - 1;
        repeat_n(operators, placements)
            .multi_cartesian_product()
            .any(|ops| {
                // start with first operand
                let mut acc: Int = *self.operands.first().unwrap();

                // fold through rest of operands
                self.operands
                    .iter()
                    .skip(1)
                    .zip(ops)
                    .for_each(|(operand, operator)| acc = operator(acc, *operand));

                acc == self.target
            })
    }
}

pub fn part_one(input: &str) -> Option<Int> {
    let ops = vec![|x, y| x + y, |x, y| x * y];
    let result: Int = input
        .par_lines()
        .filter(|s| !s.is_empty())
        .map(Calibration::from_str)
        .filter_map(|c| match c.test_operators_produce_target(&ops) {
            true => Some(c.target),
            false => None,
        })
        .sum::<Int>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<Int> {
    let ops = vec![|x, y| x + y, |x, y| x * y, |x: Int, y: Int| {
        (x * 10u64.pow(y.ilog10() + 1)) + y
    }];
    let result: Int = input
        .par_lines()
        .filter(|s| !s.is_empty())
        .map(Calibration::from_str)
        .filter_map(|c| match c.test_operators_produce_target(&ops) {
            true => Some(c.target),
            false => None,
        })
        .sum::<Int>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

use fxhash::FxHashMap;
use itertools::Itertools;

advent_of_code::solution!(11);

type Int = u64;

struct StoneRow {
    initial_stones: Vec<String>,
    stone_blink_results: FxHashMap<(String, usize), Int>,
}

impl StoneRow {
    fn from_str(input: &str) -> Self {
        Self {
            initial_stones: input.trim().split(" ").map(str::to_owned).collect_vec(),
            stone_blink_results: FxHashMap::default(),
        }
    }

    fn count_stones_after_blinks(&mut self, stone: String, remaining_blinks: usize) -> Int {
        if remaining_blinks == 0 {
            return 1;
        }

        let key = (stone.to_owned(), remaining_blinks);
        match self.stone_blink_results.get(&key) {
            None => {
                let count = Self::blink_stone(stone.as_str())
                    .iter()
                    .map(|new_stone| {
                        self.count_stones_after_blinks(new_stone.to_owned(), remaining_blinks - 1)
                    })
                    .sum();
                let _ = self.stone_blink_results.insert(key, count);
                count
            }
            Some(c) => *c,
        }
    }

    fn blink_stone(stone_num: &str) -> Vec<String> {
        match stone_num {
            "0" | "" => vec!["1".to_owned()],
            num if num.len() % 2 == 0 => {
                let new_nums = num.split_at(num.len() / 2);
                vec![
                    new_nums.0.to_owned(),
                    new_nums.1.trim_start_matches('0').to_owned(),
                ]
            }
            num => vec![(num.parse::<Int>().unwrap() * 2024).to_string()],
        }
    }

    fn count_all_after_blinks(&mut self, blinks: usize) -> Int {
        let mut sum: Int = 0;
        for i in 0..self.initial_stones.len() {
            sum += self
                .count_stones_after_blinks(self.initial_stones.get(i).unwrap().to_owned(), blinks);
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(StoneRow::from_str(input).count_all_after_blinks(25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(StoneRow::from_str(input).count_all_after_blinks(75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}

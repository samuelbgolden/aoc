advent_of_code::solution!(3);

use regex::Regex;

fn parse_and_mult(x: &str, y: &str) -> u32 {
    x.parse::<u32>().expect("parse x") * y.parse::<u32>().expect("parse y")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mul_matcher = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("regex compiles");
    let mut sum: u32 = 0;
    for (_, [x, y]) in mul_matcher.captures_iter(input).map(|c| c.extract()) {
        sum += parse_and_mult(x, y);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let toggleable_mul_matcher =
        Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").expect("regex compiles");
    let mut sum: u32 = 0;
    let mut enabled: bool = true;
    for cap in toggleable_mul_matcher.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            s if s.starts_with("m") => {
                if enabled {
                    sum +=
                        parse_and_mult(cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());
                }
            }
            s if s.starts_with("don") => {
                enabled = false;
            }
            s if s.starts_with("do") => {
                enabled = true;
            }
            _ => panic!("how did this happen"),
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(322));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }
}

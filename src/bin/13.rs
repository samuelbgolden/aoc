use itertools::Itertools;
use regex::{Match, Regex};

advent_of_code::solution!(13);

type Int = i64;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    pub x: Int,
    pub y: Int,
}

type Delta = Pos;

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct PrizeMachine {
    a_button: Delta,
    a_cost: u64,
    b_button: Delta,
    b_cost: u64,
    prize_pos: Pos,
}

impl PrizeMachine {
    fn new(a_button: Delta, a_cost: u64, b_button: Delta, b_cost: u64, prize_pos: Pos) -> Self {
        Self {
            a_button,
            a_cost,
            b_button,
            b_cost,
            prize_pos,
        }
    }

    fn make_big(&mut self) {
        self.prize_pos.x += 10000000000000;
        self.prize_pos.y += 10000000000000;
    }

    fn solve(&self) -> Option<u64> {
        let b_solution = ((self.a_button.x * self.prize_pos.y)
            - (self.prize_pos.x * self.a_button.y))
            / ((self.a_button.x * self.b_button.y) - (self.b_button.x * self.a_button.y));
        let a_solution = (self.prize_pos.x - (self.b_button.x * b_solution)) / self.a_button.x;
        if ((b_solution * self.b_button.x) + (a_solution * self.a_button.x) == self.prize_pos.x)
            && ((b_solution * self.b_button.y) + (a_solution * self.a_button.y) == self.prize_pos.y)
        {
            return Some((b_solution as u64 * self.b_cost) + (a_solution as u64 * self.a_cost));
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machine_matcher: Regex = Regex::new(
        r"A: X\+(\d*), Y\+(\d*).*\n.*B: X\+(\d*), Y\+(\d*).*\n.*Prize: X=(\d*), Y=(\d*)",
    )
    .unwrap();
    let machines: Vec<PrizeMachine> = input
        .split("\n\n")
        .map(|machine_str| {
            let captures = machine_matcher.captures(machine_str).unwrap();
            PrizeMachine::new(
                Pos {
                    x: convert(captures.get(1)),
                    y: convert(captures.get(2)),
                },
                3,
                Pos {
                    x: convert(captures.get(3)),
                    y: convert(captures.get(4)),
                },
                1,
                Pos {
                    x: convert(captures.get(5)),
                    y: convert(captures.get(6)),
                },
            )
        })
        .collect_vec();
    Some(machines.iter().filter_map(PrizeMachine::solve).sum())
}

fn convert(s: Option<Match>) -> Int {
    s.unwrap().as_str().parse::<Int>().unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let machine_matcher: Regex = Regex::new(
        r"A: X\+(\d*), Y\+(\d*).*\n.*B: X\+(\d*), Y\+(\d*).*\n.*Prize: X=(\d*), Y=(\d*)",
    )
    .unwrap();
    let mut machines: Vec<PrizeMachine> = input
        .split("\n\n")
        .map(|machine_str| {
            let captures = machine_matcher.captures(machine_str).unwrap();
            PrizeMachine::new(
                Pos {
                    x: convert(captures.get(1)),
                    y: convert(captures.get(2)),
                },
                3,
                Pos {
                    x: convert(captures.get(3)),
                    y: convert(captures.get(4)),
                },
                1,
                Pos {
                    x: convert(captures.get(5)),
                    y: convert(captures.get(6)),
                },
            )
        })
        .collect_vec();
    Some(
        machines
            .iter_mut()
            .filter_map(|m| {
                m.make_big();
                m.solve()
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}

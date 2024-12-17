use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Match, Regex};

advent_of_code::solution!(14);

type Int = i32;

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

fn convert(s: Option<Match>) -> Int {
    s.unwrap().as_str().parse::<Int>().unwrap()
}

struct Robot {
    pos: Pos,
    vel: Delta,
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let caps = INPUT_MATCHER.captures(s).unwrap();
        Self {
            pos: Pos {
                x: convert(caps.get(1)),
                y: convert(caps.get(2)),
            },
            vel: Delta {
                x: convert(caps.get(3)),
                y: convert(caps.get(4)),
            },
        }
    }

    fn step(&mut self, x_bound: Int, y_bound: Int) {
        self.pos = self.pos + self.vel;
        if self.pos.x < 0 {
            self.pos.x += x_bound;
        } else if self.pos.x >= x_bound {
            self.pos.x -= x_bound;
        }

        if self.pos.y < 0 {
            self.pos.y += y_bound;
        } else if self.pos.y >= y_bound {
            self.pos.y -= y_bound;
        }
    }

    fn step_n(&mut self, n: usize, x_bound: Int, y_bound: Int) {
        for _ in 0..n {
            self.step(x_bound, y_bound);
        }
    }
}

lazy_static! {
    static ref INPUT_MATCHER: Regex =
        Regex::new(r"p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)").unwrap();
}

pub fn part_one(input: &str) -> Option<u32> {
    let w = 11;
    let h = 7;
    let bot_positions = input
        .lines()
        .map(|l| {
            let mut robot = Robot::from_str(l);
            robot.step_n(100, w, h);
            robot.pos
        })
        .collect_vec();
    let quad_cnts = bot_positions
        .iter()
        .fold((0, 0, 0, 0), |(q1, q2, q3, q4), pos| match pos {
            Pos { x, y } if *x < (w / 2) && *y < (h / 2) => (q1 + 1, q2, q3, q4),
            Pos { x, y } if *x > (w / 2) && *y < (h / 2) => (q1, q2 + 1, q3, q4),
            Pos { x, y } if *x < (w / 2) && *y > (h / 2) => (q1, q2, q3 + 1, q4),
            Pos { x, y } if *x > (w / 2) && *y > (h / 2) => (q1, q2, q3, q4 + 1),
            _ => (q1, q2, q3, q4),
        });
    Some(quad_cnts.0 * quad_cnts.1 * quad_cnts.2 * quad_cnts.3)
}

pub fn part_two(input: &str) -> Option<u32> {
    let w = 101;
    let h = 103;
    let mut bots: Vec<Robot> = input.lines().map(Robot::from_str).collect_vec();
    let mut min_sec_val = (-1 as Int, Int::MAX);
    for i in 0..10000 {
        let quad_cnts = bots
            .iter()
            .fold((0, 0, 0, 0), |(q1, q2, q3, q4), bot| match bot.pos {
                Pos { x, y } if x < (w / 2) && y < (h / 2) => (q1 + 1, q2, q3, q4),
                Pos { x, y } if x > (w / 2) && y < (h / 2) => (q1, q2 + 1, q3, q4),
                Pos { x, y } if x < (w / 2) && y > (h / 2) => (q1, q2, q3 + 1, q4),
                Pos { x, y } if x > (w / 2) && y > (h / 2) => (q1, q2, q3, q4 + 1),
                _ => (q1, q2, q3, q4),
            });
        //let mut grid: [[usize; 101]; 103] = [[0; 101]; 103];
        //bots.iter().for_each(|bot| grid[bot.pos.y as usize][bot.pos.x as usize] += 1);
        //println!("step: {}\n{}\n", i, grid.iter().map(|row| row.iter().map(|cell| if *cell == 0 { ".".to_string() } else { "#".to_string() }).join(" ")).join("\n"));
        let sec_val = quad_cnts.0 * quad_cnts.1 * quad_cnts.2 * quad_cnts.3;
        if sec_val < min_sec_val.1 {
            min_sec_val = (i as Int, sec_val);
        }
        bots.iter_mut().for_each(|bot| bot.step(w, h));
    }
    println!("{}", min_sec_val.0);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

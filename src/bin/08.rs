use std::{
    collections::HashMap,
    hash::BuildHasherDefault,
    ops::{Add, Sub},
};

use fxhash::{FxHashSet, FxHasher};
use itertools::Itertools;

advent_of_code::solution!(8);

type Int = i32;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    pub x: Int,
    pub y: Int,
}

impl Pos {
    fn new(x: Int, y: Int) -> Self {
        Self { x, y }
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn build_antenna_map(
    input: &str,
) -> (
    Int,
    Int,
    HashMap<Int, Vec<Pos>, BuildHasherDefault<FxHasher>>,
) {
    let height = input.lines().count() as Int;
    let width = input.lines().next().unwrap().chars().count() as Int;
    let mut antenna_map: HashMap<Int, Vec<Pos>, BuildHasherDefault<FxHasher>> = HashMap::default();
    input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    let pos = Pos::new(x as Int, y as Int);
                    antenna_map.entry(c as Int).or_default().push(pos)
                }
            })
        });
    (width, height, antenna_map)
}

fn calc_antinodes(a: Pos, b: Pos) -> Vec<Pos> {
    vec![a - (b - a), b - (a - b)]
}

fn calc_resonant_antinodes(a: Pos, b: Pos, width: Int, height: Int) -> Vec<Pos> {
    let mut result: Vec<Pos> = Vec::new();

    let a_diff = b - a;
    let mut potential_antinode = a;
    loop {
        if is_in_bounds(width, height, potential_antinode) {
            result.push(potential_antinode);
            potential_antinode = potential_antinode - a_diff;
        } else {
            break;
        }
    }

    let b_diff = a - b;
    potential_antinode = b;
    loop {
        if is_in_bounds(width, height, potential_antinode) {
            result.push(potential_antinode);
            potential_antinode = potential_antinode - b_diff;
        } else {
            break;
        }
    }

    result
}

fn is_in_bounds(width: Int, height: Int, pos: Pos) -> bool {
    pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height
}

pub fn part_one(input: &str) -> Option<u32> {
    let (width, height, map) = build_antenna_map(input);
    let valid_antinodes: FxHashSet<Pos> = map
        .values()
        .flat_map(|same_antennae| {
            same_antennae
                .iter()
                .tuple_combinations::<(_, _)>()
                .flat_map(|(a, b)| calc_antinodes(*a, *b))
        })
        .filter(|pos| is_in_bounds(width, height, *pos))
        .collect();
    Some(valid_antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (width, height, map) = build_antenna_map(input);
    let valid_antinodes: FxHashSet<Pos> = map
        .values()
        .flat_map(|same_antennae| {
            same_antennae
                .iter()
                .tuple_combinations::<(_, _)>()
                .flat_map(|(a, b)| calc_resonant_antinodes(*a, *b, width, height))
        })
        .collect();
    Some(valid_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

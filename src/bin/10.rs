use std::ops::{Add, Sub};

use fxhash::FxHashSet;
use itertools::Itertools;

advent_of_code::solution!(10);

type Int = i16;
type Alt = usize;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Pos {
    pub x: Int,
    pub y: Int,
}

const NEIGHBORS: &[Pos] = &[
    Pos { x: 1, y: 0 },
    Pos { x: 0, y: 1 },
    Pos { x: -1, y: 0 },
    Pos { x: 0, y: -1 },
];

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

#[derive(Clone, Debug)]
struct TopoMap {
    map: Vec<Vec<Alt>>,
    trailheads: Vec<Pos>,
}

impl TopoMap {
    fn from_str(input: &str) -> Self {
        let mut trailheads: Vec<Pos> = Vec::new();
        let map: Vec<Vec<Alt>> = input
            .split('\n')
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '0' => {
                            trailheads.push(Pos::new(x as Int, y as Int));
                            0
                        }
                        _ => c.to_digit(10).unwrap() as Alt,
                    })
                    .collect::<Vec<Alt>>()
            })
            .collect();
        let row_len = map.first().unwrap().len() as i32;
        assert!(map.iter().skip(1).all(|row| (row.len() as i32) == row_len));
        Self {
            map,
            trailheads,
        }
    }

    fn get_altitude(&self, pos: &Pos) -> Option<&Alt> {
        self.map.get(pos.y as usize)?.get(pos.x as usize)
    }

    fn get_trail_score(&self, trailhead: &Pos) -> Int {
        //println!("trailhead {:?}", trailhead);
        let mut traversal = vec![*trailhead];
        let mut counter = 0;
        while !traversal.is_empty() && counter != 9 {
            //println!("\ttraversal has {:?}", traversal);
            counter += 1;
            traversal = traversal
                .iter()
                .flat_map(|pos| {
                    NEIGHBORS.iter().filter_map(|neighbor_delta| {
                        let new_pos = *pos + *neighbor_delta;
                        match self.get_altitude(&new_pos) {
                            Some(alt) if *alt == counter => Some(new_pos),
                            _ => None,
                        }
                    })
                })
                .dedup()
                .collect_vec();
        }
        return FxHashSet::from_iter(traversal.iter()).len() as Int;
    }

    fn get_trail_score_sum(&self) -> Int {
        self.trailheads
            .iter()
            .map(|t| self.get_trail_score(t))
            .sum()
    }

    fn get_distinct_trail_score_rec(&self, pos: Pos, curr_alt: Alt) -> Int {
        match self.get_altitude(&pos) {
            None => 0,
            Some(9) if curr_alt == 9 => 1,
            Some(alt) if curr_alt == *alt => NEIGHBORS
                .iter()
                .map(|neighbor_delta| {
                    self.get_distinct_trail_score_rec(pos + *neighbor_delta, curr_alt + 1)
                })
                .sum(),
            _ => 0,
        }
    }

    fn get_distinct_trail_score(&self, pos: Pos) -> Int {
        self.get_distinct_trail_score_rec(pos, 0)
    }

    fn get_distinct_trail_score_sum(&self) -> Int {
        self.trailheads
            .iter()
            .map(|t| self.get_distinct_trail_score(*t))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let topo_map = TopoMap::from_str(input);
    Some(topo_map.get_trail_score_sum() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let topo_map = TopoMap::from_str(input);
    Some(topo_map.get_distinct_trail_score_sum() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}

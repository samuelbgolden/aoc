use std::{
    collections::VecDeque,
    ops::{Add, Sub},
};

use fxhash::FxHashSet;

advent_of_code::solution!(12);

type Int = i16;

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

const VERTICAL_ADJ_SIDE: (Pos, Pos) = (Pos::new(0, -1), Pos::new(0, 1));
const HORIZONTAL_ADJ_SIDE: (Pos, Pos) = (Pos::new(-1, 0), Pos::new(1, 0));

impl Pos {
    const fn new(x: Int, y: Int) -> Self {
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
struct GardenMap {
    map: Vec<Vec<char>>,
}

impl GardenMap {
    fn from_str(input: &str) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let row_len = map.first().unwrap().len() as i32;
        assert!(map.iter().skip(1).all(|row| (row.len() as i32) == row_len));
        Self { map }
    }

    fn get(&self, pos: Pos) -> Option<&char> {
        self.map.get(pos.y as usize)?.get(pos.x as usize)
    }

    fn get_fencing_price_perimeter(&self) -> u32 {
        let mut added_to_region: FxHashSet<Pos> = FxHashSet::default();
        let mut price_sum = 0u32;
        for x in 0..self.map.first().unwrap().len() {
            for y in 0..self.map.len() {
                let init_pos = Pos::new(x as Int, y as Int);
                if added_to_region.contains(&init_pos) {
                    continue;
                }
                let (region, borders) = self.get_region_and_border_count_starting_at(init_pos);
                price_sum += region.len() as u32 * borders;
                added_to_region.extend(region);
            }
        }
        price_sum
    }

    fn get_region_and_border_count_starting_at(&self, init_pos: Pos) -> (FxHashSet<Pos>, u32) {
        let plant = self.get(init_pos).unwrap();
        let mut region = FxHashSet::default();
        let mut border_count = 0;
        let mut candidates: VecDeque<Pos> = VecDeque::new();
        candidates.push_back(init_pos);
        while let Some(pos) = candidates.pop_front() {
            if region.contains(&pos) {
                continue;
            }
            NEIGHBORS.iter().for_each(|n_delta| {
                let n_pos = pos + *n_delta;
                match self.get(n_pos) {
                    Some(p) if p == plant => candidates.push_back(n_pos),
                    _ => border_count += 1,
                }
            });
            region.insert(pos);
        }
        (region, border_count)
    }

    fn get_fencing_price_sides(&self) -> u32 {
        let mut added_to_region: FxHashSet<Pos> = FxHashSet::default();
        let mut price_sum = 0u32;
        for x in 0..self.map.first().unwrap().len() {
            for y in 0..self.map.len() {
                let init_pos = Pos::new(x as Int, y as Int);
                if added_to_region.contains(&init_pos) {
                    continue;
                }
                let (region, borders) = self.get_region_and_borders_starting_at(init_pos);
                let side_count = Self::count_sides(borders);
                price_sum += region.len() as u32 * side_count;
                added_to_region.extend(region);
            }
        }
        price_sum
    }

    fn count_sides(pairs: FxHashSet<(Pos, Pos)>) -> u32 {
        // copy the pairs
        let mut border_pairs = pairs.to_owned();
        // start with no sides
        let mut side_count = 0u32;
        // while there is still a border not found in a side, keep counting sides
        while let Some((a, b)) = border_pairs.iter().next() {
            // start with the next border pair available
            let mut side_pairs: Vec<(Pos, Pos)> = Vec::new();
            let mut side_pairs_to_vis: VecDeque<(Pos, Pos)> = VecDeque::new();
            side_pairs_to_vis.push_back((*a, *b));
            while let Some((c, d)) = side_pairs_to_vis.pop_front() {
                // get the borders which could be immediately adjacent this side
                let possible_adj_sides = match c - d {
                    Pos { x: 0, y: _ } => vec![
                        (c + HORIZONTAL_ADJ_SIDE.0, d + HORIZONTAL_ADJ_SIDE.0),
                        (c + HORIZONTAL_ADJ_SIDE.1, d + HORIZONTAL_ADJ_SIDE.1),
                    ],
                    Pos { x: _, y: 0 } => vec![
                        (c + VERTICAL_ADJ_SIDE.0, d + VERTICAL_ADJ_SIDE.0),
                        (c + VERTICAL_ADJ_SIDE.1, d + VERTICAL_ADJ_SIDE.1),
                    ],
                    _ => panic!("positions not adjacent: {:?}, {:?}", a, b),
                };
                // if they are indeed in the border pairs, add them to this side
                possible_adj_sides.iter().for_each(|pair| {
                    if border_pairs.contains(pair) && !side_pairs.contains(pair) {
                        side_pairs_to_vis.push_back(*pair);
                    }
                });
                side_pairs.push((c, d));
            }
            // remove all of these borders from the border_pairs
            side_pairs.iter().for_each(|p| {
                border_pairs.remove(p);
            });
            side_count += 1;
        }
        side_count
    }

    fn get_region_and_borders_starting_at(
        &self,
        init_pos: Pos,
    ) -> (FxHashSet<Pos>, FxHashSet<(Pos, Pos)>) {
        let plant = self.get(init_pos).unwrap();
        let mut region = FxHashSet::default();
        let mut borders: FxHashSet<(Pos, Pos)> = FxHashSet::default();
        let mut candidates: VecDeque<Pos> = VecDeque::new();
        candidates.push_back(init_pos);
        while let Some(pos) = candidates.pop_front() {
            if region.contains(&pos) {
                continue;
            }
            NEIGHBORS.iter().for_each(|n_delta| {
                let n_pos = pos + *n_delta;
                match self.get(n_pos) {
                    Some(p) if p == plant => candidates.push_back(n_pos),
                    _ => {
                        let _ = borders.insert((pos, n_pos));
                    }
                }
            });
            region.insert(pos);
        }
        (region, borders)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(GardenMap::from_str(input).get_fencing_price_perimeter())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(GardenMap::from_str(input).get_fencing_price_sides())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}

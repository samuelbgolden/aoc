use std::{collections::HashSet, fmt::Display, hash::BuildHasherDefault, ops::Add};

use fxhash::FxHasher;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    pub x: i16,
    pub y: i16,
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

type Dir = Pos;
type Hasher = FxHasher;

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
}

enum WalkState {
    InProgress,
    OutOfBounds,
    Looping,
}

struct PatrolMap {
    map: Vec<Vec<Tile>>,
    guard_pos: Pos,
    init_guard_pos: Pos,
    guard_dir: Dir,
    init_guard_dir: Dir,
    visited: HashSet<(Pos, Dir), BuildHasherDefault<Hasher>>,
}

impl PatrolMap {
    fn from_str(input: &str) -> Self {
        let mut guard_pos: Option<Pos> = None;
        let map: Vec<Vec<Tile>> = input
            .split('\n')
            .enumerate()
            .filter(|(_, s)| !s.is_empty())
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        '^' => {
                            guard_pos = Some(Pos {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            });
                            Tile::Empty
                        }
                        other => panic!("unexpected char in input {}", other),
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();
        let row_len = map.first().unwrap().len() as i32;
        assert!(map.iter().skip(1).all(|row| (row.len() as i32) == row_len));
        Self {
            map,
            guard_pos: guard_pos.expect("guard not found in input"),
            init_guard_pos: guard_pos.expect("guard not found in input"),
            guard_dir: Dir { x: 0, y: -1 },
            init_guard_dir: Dir { x: 0, y: -1 },
            visited: HashSet::<(Pos, Dir), BuildHasherDefault<Hasher>>::default(),
        }
    }

    fn get_tile(&self, pos: Pos) -> Option<&Tile> {
        self.map.get(pos.y as usize)?.get(pos.x as usize)
    }

    fn step(&mut self) -> WalkState {
        let next_pos = self.guard_pos + self.guard_dir;
        match self.get_tile(next_pos) {
            Some(Tile::Wall) => {
                self.turn();
                self.step()
            }
            Some(Tile::Empty) => {
                let result = match self.visited.insert((self.guard_pos, self.guard_dir)) {
                    false => WalkState::Looping,
                    true => WalkState::InProgress,
                };
                self.guard_pos = next_pos;
                result
            }
            None => {
                self.visited.insert((self.guard_pos, self.guard_dir));
                WalkState::OutOfBounds
            }
        }
    }

    fn turn(&mut self) {
        self.guard_dir = match &self.guard_dir {
            Dir { x: -1, y: 0 } => Dir { x: 0, y: -1 },
            Dir { x: 0, y: -1 } => Dir { x: 1, y: 0 },
            Dir { x: 1, y: 0 } => Dir { x: 0, y: 1 },
            Dir { x: 0, y: 1 } => Dir { x: -1, y: 0 },
            other => panic!("unexpected guard direction {:?}", other),
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.guard_pos = self.init_guard_pos;
        self.guard_dir = self.init_guard_dir;
    }

    fn is_patrol_loop(&mut self, print: bool) -> bool {
        self.reset();
        loop {
            match self.step() {
                WalkState::InProgress => (),
                WalkState::OutOfBounds => return false,
                WalkState::Looping => return true,
            };
            if print {
                println!("{}\n", self);
            }
        }
    }

    fn count_visited_on_patrol(&mut self, print: bool) -> u32 {
        let _ = self.is_patrol_loop(print);
        self.visited
            .iter()
            .map(|(pos, _)| *pos)
            .collect::<HashSet<Pos>>()
            .len() as u32
    }

    fn is_patrol_loop_with_obstacle(&mut self, obstacle_pos: Pos, print: bool) -> Result<bool, ()> {
        if *self.get_tile(obstacle_pos).ok_or(())? == Tile::Wall {
            return Err(());
        }
        *self
            .map
            .get_mut(obstacle_pos.y as usize)
            .ok_or(())?
            .get_mut(obstacle_pos.x as usize)
            .ok_or(())? = Tile::Wall;
        let test = self.is_patrol_loop(print);
        *self
            .map
            .get_mut(obstacle_pos.y as usize)
            .ok_or(())?
            .get_mut(obstacle_pos.x as usize)
            .ok_or(())? = Tile::Empty;
        Ok(test)
    }

    fn count_new_obstacle_loops(&mut self, print: bool) -> u32 {
        self.is_patrol_loop(print);
        // only try for obstacle positions the patrol will actually encounter
        self.visited
            .clone()
            .iter()
            .filter_map(|(pos, dir)| {
                let obstacle_pos = *pos + *dir;
                match self.is_patrol_loop_with_obstacle(obstacle_pos, print) {
                    Ok(true) => Some(obstacle_pos),
                    _ => None,
                }
            })
            .collect::<HashSet<Pos>>()
            .len() as u32
    }
}

impl Display for PatrolMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let visited_pos: HashSet<Pos> = self.visited.iter().map(|(pos, _)| *pos).collect();
        write!(
            f,
            "{}",
            self.map
                .iter()
                .enumerate()
                .map(|(y, row)| row
                    .iter()
                    .enumerate()
                    .map(|(x, t)| {
                        if visited_pos.contains(
                            &(Pos {
                                x: x as i16,
                                y: y as i16,
                            }),
                        ) {
                            return 'o';
                        }
                        match t {
                            Tile::Empty => '.',
                            Tile::Wall => '#',
                        }
                    })
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut patrol = PatrolMap::from_str(input);
    Some(patrol.count_visited_on_patrol(false))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut patrol = PatrolMap::from_str(input);
    Some(patrol.count_new_obstacle_loops(false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

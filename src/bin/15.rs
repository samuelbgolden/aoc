use std::fmt::Display;

use itertools::Itertools;

advent_of_code::solution!(15);

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

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

#[derive(Clone, Debug)]
enum Tile {
    Empty,
    Wall,
    BigBoxLeft,
    BigBoxRight,
    Box,
    Robot,
}

#[derive(Clone, Debug)]
struct WarehouseMap {
    map: Vec<Vec<Tile>>,
    bot_pos: Pos,
}

impl Display for WarehouseMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.map
                .iter()
                .map(|line| line
                    .iter()
                    .map(|t| match t {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Box => 'O',
                        Tile::BigBoxRight => ']',
                        Tile::BigBoxLeft => '[',
                        Tile::Robot => '@',
                    })
                    .join(" "))
                .join("\n")
        )
    }
}

impl WarehouseMap {
    fn from_doubled_str(input: &str) -> Self {
        let mut bot_pos: Pos = Pos { x: 0, y: 0 };
        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(x, c)| match c {
                        '.' => vec![Tile::Empty, Tile::Empty],
                        '#' => vec![Tile::Wall, Tile::Wall],
                        'O' => vec![Tile::BigBoxLeft, Tile::BigBoxRight],
                        '@' => {
                            bot_pos.x = TryInto::<Int>::try_into(x).unwrap() * 2;
                            bot_pos.y = y.try_into().unwrap();
                            vec![Tile::Robot, Tile::Empty]
                        }
                        _ => panic!("wtf is this: {}", c),
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();
        let row_len = map.first().unwrap().len() as i32;
        assert!(map.iter().skip(1).all(|row| (row.len() as i32) == row_len));
        Self { map, bot_pos }
    }

    fn from_str(input: &str) -> Self {
        let mut bot_pos: Pos = Pos { x: 0, y: 0 };
        let map: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty())
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        'O' => Tile::Box,
                        '@' => {
                            bot_pos.x = x.try_into().unwrap();
                            bot_pos.y = y.try_into().unwrap();
                            Tile::Robot
                        }
                        _ => panic!("wtf is this: {}", c),
                    })
                    .collect()
            })
            .collect();
        let row_len = map.first().unwrap().len() as i32;
        assert!(map.iter().skip(1).all(|row| (row.len() as i32) == row_len));
        Self { map, bot_pos }
    }

    fn get(&self, pos: Pos) -> Option<&Tile> {
        self.map.get(pos.y as usize)?.get(pos.x as usize)
    }

    fn shift(&mut self, pos: &Pos, delta: &Delta) {
        let new_pos: Pos = *pos + *delta;
        *self
            .map
            .get_mut(new_pos.y as usize)
            .expect("bad y")
            .get_mut(new_pos.x as usize)
            .expect("bad x") = self.get(*pos).expect("bad pos").clone();
        *self
            .map
            .get_mut(pos.y as usize)
            .expect("bad y")
            .get_mut(pos.x as usize)
            .expect("bad x") = Tile::Empty;
    }

    fn push(&mut self, delta: Delta) {
        let mut to_move: Vec<Pos> = vec![self.bot_pos];
        // (<position in lane to check>, <whether it is known to be clear/blocked>)
        let mut maybe_blocked_lanes: Vec<(Pos, Option<bool>)> = vec![(self.bot_pos, None)];
        let mut new_maybe_blocked: Vec<Pos> = vec![];
        loop {
            for (curr_pos, is_clear) in maybe_blocked_lanes
                .iter_mut()
                .filter(|(_, is_clear)| is_clear.is_none())
            {
                *curr_pos += delta;
                match self.get(*curr_pos) {
                    Some(Tile::BigBoxLeft) if delta.y != 0 => {
                        if !to_move.contains(curr_pos) {
                            to_move.push(*curr_pos);
                        }
                        let other_half_pos = *curr_pos + Pos { x: 1, y: 0 };
                        if !to_move.contains(&other_half_pos) {
                            to_move.push(other_half_pos);
                            new_maybe_blocked.push(other_half_pos);
                        }
                    }
                    Some(Tile::BigBoxRight) if delta.y != 0 => {
                        if !to_move.contains(curr_pos) {
                            to_move.push(*curr_pos);
                        }
                        let other_half_pos = *curr_pos + Pos { x: -1, y: 0 };
                        if !to_move.contains(&other_half_pos) {
                            to_move.push(*curr_pos + Pos { x: -1, y: 0 });
                            new_maybe_blocked.push(other_half_pos);
                        }
                    }
                    Some(Tile::Box) | Some(Tile::BigBoxRight) | Some(Tile::BigBoxLeft) => {
                        to_move.push(*curr_pos)
                    }
                    Some(Tile::Wall) => *is_clear = Some(false),
                    Some(Tile::Empty) => *is_clear = Some(true),
                    other => panic!("what do: {:?} @ {:?}", other, curr_pos),
                }
            }
            if !new_maybe_blocked.is_empty() {
                new_maybe_blocked
                    .iter()
                    .for_each(|p| maybe_blocked_lanes.push((*p, None)));
                new_maybe_blocked.clear();
            } else if maybe_blocked_lanes
                .iter()
                .any(|(_, is_clear)| *is_clear == Some(false))
            {
                // do nothing
                break;
            } else if maybe_blocked_lanes
                .iter()
                .all(|(_, is_clear)| *is_clear == Some(true))
            {
                self.bot_pos += delta;
                to_move
                    .iter()
                    .rev()
                    .dedup()
                    .for_each(|p| self.shift(p, &delta));
                break;
            }
        }
    }

    fn sum_gps(&self) -> u32 {
        let mut sum = 0u32;
        for y in 0..self.map.len() {
            for x in 0..self.map.first().unwrap().len() {
                if let Some(Tile::Box) | Some(Tile::BigBoxLeft) = self.get(Pos {
                    x: x as Int,
                    y: y as Int,
                }) {
                    sum += (x as u32) + (100 * (y as u32));
                }
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let mut warehouse = WarehouseMap::from_str(sections.next()?);
    for c in sections.next()?.chars() {
        match c {
            '^' => warehouse.push(Pos { x: 0, y: -1 }),
            '>' => warehouse.push(Pos { x: 1, y: 0 }),
            'v' => warehouse.push(Pos { x: 0, y: 1 }),
            '<' => warehouse.push(Pos { x: -1, y: 0 }),
            '\n' => (),
            _ => panic!("idk what to do with '{}'", c),
        }
        //println!("push {}:\n{}\n", c, warehouse);
    }
    Some(warehouse.sum_gps())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sections = input.split("\n\n");
    let mut warehouse = WarehouseMap::from_doubled_str(sections.next()?);
    for c in sections.next()?.chars() {
        match c {
            '^' => warehouse.push(Pos { x: 0, y: -1 }),
            '>' => warehouse.push(Pos { x: 1, y: 0 }),
            'v' => warehouse.push(Pos { x: 0, y: 1 }),
            '<' => warehouse.push(Pos { x: -1, y: 0 }),
            '\n' => (),
            _ => panic!("idk what to do with '{}'", c),
        }
        //sleep_ms(300);
        //println!("push {}:\n{}\n", c, warehouse);
    }
    Some(warehouse.sum_gps())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}

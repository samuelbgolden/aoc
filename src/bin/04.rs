advent_of_code::solution!(4);

struct WordSearch {
    letters: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

const DELTAS: &[(i16, i16)] = &[
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const X_DELTAS: &[(i16, i16)] = &[(1, 1), (-1, 1), (-1, -1), (1, -1)];

impl WordSearch {
    fn from_str(input: &str) -> Self {
        let letters: Vec<Vec<char>> = input
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let row_len = letters.first().unwrap().len();
        assert!(letters.iter().skip(1).all(|row| row.len() == row_len));
        Self {
            width: row_len,
            height: letters.len(),
            letters,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.letters.get(y)?.get(x)
    }

    fn find_word_count(&self, word: &str) -> u32 {
        let mut count_found: u32 = 0;
        for sy in 0..self.height {
            let y = sy as i16;
            for sx in 0..self.width {
                let x = sx as i16;
                // check first letter
                if let Some(c) = self.get(x.try_into().unwrap(), y.try_into().unwrap()) {
                    if *c != word.chars().nth(0).unwrap() {
                        continue;
                    }
                }

                // check each direction with remaining letters
                count_found += DELTAS
                    .iter()
                    .filter(|(dx, dy)| self.is_word_at_ray(word, x, y, *dx, *dy))
                    .count() as u32;
            }
        }
        count_found
    }

    fn find_word_x_count(&self, word: &str) -> u32 {
        assert!(word.len() % 2 != 0);
        let word_dist = (word.len() - 1) as i16;
        let mut count_found: u32 = 0;
        for sy in 0..self.height {
            let y = sy as i16;
            for sx in 0..self.width {
                let x = sx as i16;
                // check first letter
                if let Some(c) = self.get(x.try_into().unwrap(), y.try_into().unwrap()) {
                    if *c != word.chars().nth(0).unwrap() {
                        continue;
                    }
                }

                // check possible cross rays direction with remaining letters
                X_DELTAS
                    .iter()
                    // get instances of word
                    .filter(|(dx, dy)| self.is_word_at_ray(word, x, y, *dx, *dy))
                    // check possible crossing words
                    .for_each(|(dx, dy)| {
                        if self.is_word_at_ray(word, x + (*dx * word_dist), y, *dx * -1, *dy)
                            || self.is_word_at_ray(word, x, y + (*dy * word_dist), *dx, *dy * -1)
                        {
                            count_found += 1;
                        }
                    });
            }
        }

        count_found / 2
    }

    fn is_word_at_ray(&self, word: &str, start_x: i16, start_y: i16, dx: i16, dy: i16) -> bool {
        (0..word.len()).all(|w_idx| {
            if let Some(c) = self.get(
                (start_x + (dx * w_idx as i16)) as usize,
                (start_y + (dy * w_idx as i16)) as usize,
            ) {
                return *c == word.chars().nth(w_idx).unwrap();
            }
            false
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(WordSearch::from_str(input).find_word_count("XMAS"))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(WordSearch::from_str(input).find_word_x_count("MAS"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

advent_of_code::solution!(2);

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

#[allow(dead_code)]
impl Report {
    fn from_str(input_line: &str) -> Self {
        Self {
            levels: input_line
                .split_ascii_whitespace()
                .map(|s| {
                    s.parse::<u32>()
                        .unwrap_or_else(|_| panic!("could not parse '{}'", s))
                })
                .collect(),
        }
    }

    fn from_vec(levels: Vec<u32>) -> Self {
        Self { levels }
    }

    fn check_safe(&self) -> bool {
        let mut is_ascending: Option<bool> = None;
        for nums in self.levels.windows(2) {
            if !Report::has_valid_ordering(nums[0], nums[1], &mut is_ascending) {
                return false;
            };
        }
        true
    }

    fn brute_check_safe_with_removal(&self) -> bool {
        if self.check_safe() {
            return true;
        }
        self.levels.iter().enumerate().any(|(i, _)| {
            let mut is_ascending: Option<bool> = None;
            let mut subvec = self.levels.clone();
            subvec.remove(i);
            for nums in subvec.windows(2) {
                if !Report::has_valid_ordering(nums[0], nums[1], &mut is_ascending) {
                    return false;
                }
            }
            true
        })
    }

    fn check_safe_with_removal(&self) -> bool {
        let mut is_ascending: Option<bool> = None;
        let mut removed_val: Option<u32> = None;

        // check if removal or last or first element is safe
        if Report::from_vec(self.levels.split_first().unwrap().1.to_vec()).check_safe()
            || Report::from_vec(self.levels.split_last().unwrap().1.to_vec()).check_safe()
        {
            return true;
        }

        // check internal elements
        let mut window_iter = self.levels.windows(3);
        while let Some(nums) = window_iter.next() {
            if Report::has_valid_ordering(nums[0], nums[1], &mut is_ascending) {
                continue;
            } else if Report::has_valid_ordering(nums[0], nums[2], &mut is_ascending)
                && removed_val.is_none()
            {
                removed_val = Some(nums[1]);
                // skip next
                let _ = window_iter.next();
            } else {
                return false;
            }
        }

        // because window size is 3, check last two elements for correct ordering
        let count = self.levels.len();
        if let Some(removed) = removed_val {
            if *self.levels.get(count - 2).unwrap() == removed {
                return Report::has_valid_ordering(
                    *self.levels.get(count - 3).unwrap(),
                    *self.levels.get(count - 1).unwrap(),
                    &mut is_ascending,
                );
            }
        }
        Report::has_valid_ordering(
            *self.levels.get(count - 2).unwrap(),
            *self.levels.get(count - 1).unwrap(),
            &mut is_ascending,
        )
    }

    fn has_valid_ordering(a: u32, b: u32, is_asc: &mut Option<bool>) -> bool {
        if !Report::is_diff_gradual(a, b) {
            return false;
        }
        if let Some(is_asc) = is_asc {
            if *is_asc != (a < b) {
                return false;
            }
        } else {
            *is_asc = Some(a < b);
        }
        true
    }

    fn is_diff_gradual(a: u32, b: u32) -> bool {
        let diff = a.abs_diff(b);
        (diff < 4) && (diff > 0)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Report> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Report::from_str)
        .collect();
    Some(reports.iter().filter(|r| r.check_safe()).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Report> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Report::from_str)
        .collect();
    Some(
        reports
            .iter()
            .filter(|r| r.brute_check_safe_with_removal())
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

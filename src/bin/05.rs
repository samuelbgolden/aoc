use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut post_rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut updates: Vec<VecDeque<usize>> = Vec::new();

    // process input
    input.split('\n').try_for_each(|line| {
        if line.contains('|') {
            let mut rule_vals = line.split('|');
            post_rules
                .entry(rule_vals.next()?.parse::<usize>().unwrap())
                .or_default()
                .insert(rule_vals.next()?.parse::<usize>().unwrap());
        } else if line.contains(',') {
            updates.push(
                line.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<VecDeque<usize>>(),
            );
        }
        Some(())
    })?;

    let mut valid_pages: Vec<usize> = Vec::new();
    let mut updates_clone = updates.clone();
    for (i, update) in updates_clone.iter_mut().enumerate() {
        let mut fails = false;
        while let Some(page) = update.pop_front() {
            if update.iter().any(|p| {
                post_rules
                    .get(p)
                    .is_some_and(|page_set| page_set.contains(&page))
            }) {
                fails = true;
                break;
            }
        }
        if !fails {
            valid_pages.push(i);
        }
    }

    Some(
        valid_pages
            .iter()
            .map(|i| {
                let update = updates.get(*i).unwrap();
                update.get(update.len() / 2).expect("midpoint num")
            })
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut post_rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut updates: Vec<VecDeque<usize>> = Vec::new();

    // process input
    input.split('\n').try_for_each(|line| {
        if line.contains('|') {
            let mut rule_vals = line.split('|');
            post_rules
                .entry(rule_vals.next()?.parse::<usize>().unwrap())
                .or_default()
                .insert(rule_vals.next()?.parse::<usize>().unwrap());
        } else if line.contains(',') {
            updates.push(
                line.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<VecDeque<usize>>(),
            );
        }
        Some(())
    })?;

    let mut invalid_pages: Vec<usize> = Vec::new();
    let mut updates_clone = updates.clone();
    for (i, update) in updates_clone.iter_mut().enumerate() {
        let mut fails = false;
        while let Some(page) = update.pop_front() {
            if update.iter().any(|p| {
                post_rules
                    .get(p)
                    .is_some_and(|page_set| page_set.contains(&page))
            }) {
                fails = true;
                break;
            }
        }
        if fails {
            invalid_pages.push(i);
        }
    }

    let mut sum: usize = 0;
    for i in invalid_pages {
        let update: &mut VecDeque<usize> = &mut updates.get(i).unwrap().clone();
        let mut sorted_update: Vec<usize> = vec![update.pop_front().unwrap()];
        while let Some(page) = update.pop_front() {
            let mut placed_idx = None;
            for j in 0..sorted_update.len() {
                if post_rules
                    .get(&page)
                    .is_some_and(|page_set| page_set.contains(sorted_update.get(j).unwrap()))
                {
                    placed_idx = Some(j);
                    break;
                }
            }
            if let Some(idx) = placed_idx {
                sorted_update.insert(idx, page);
            } else {
                sorted_update.push(page);
            }
        }
        sum += sorted_update.get(sorted_update.len() / 2).unwrap();
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

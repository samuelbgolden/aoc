use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1: Vec<i64> = vec![];
    let mut list2: Vec<i64> = vec![];
    input.split('\n').for_each(|line| {
        let vals: Vec<&str> = line.split_ascii_whitespace().collect();
        if vals.len() != 2 {
            return;
        }
        list1.push(
            vals.first()
                .expect("l1 has val")
                .parse::<i64>()
                .expect("l1 parse val"),
        );
        list2.push(
            vals.get(1)
                .expect("l2 has val")
                .parse::<i64>()
                .expect("l2 parse val"),
        );
    });

    list1.sort();
    list2.sort();

    assert!(list1.len() == list2.len());

    let mut sum: u32 = 0;
    for i in 0..list1.len() {
        sum += list1.get(i)?.abs_diff(*list2.get(i)?) as u32;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1: Vec<i64> = vec![];
    let mut freqs: HashMap<i64, i64> = HashMap::new();
    input.split('\n').for_each(|line| {
        let vals: Vec<&str> = line.split_ascii_whitespace().collect();
        if vals.len() != 2 {
            return;
        }
        list1.push(
            vals.first()
                .expect("l1 has val")
                .parse::<i64>()
                .expect("l1 parse val"),
        );
        *freqs
            .entry(
                vals.get(1)
                    .expect("l2 has val")
                    .parse::<i64>()
                    .expect("l2 parse val"),
            )
            .or_insert(0) += 1;
    });

    let mut sum: u32 = 0;
    for v in list1.iter() {
        sum += (v * freqs.get(v).unwrap_or(&0i64)) as u32;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}

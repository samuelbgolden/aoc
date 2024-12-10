use itertools::{repeat_n, Itertools};

advent_of_code::solution!(9);

fn char_to_digit(c: char) -> usize {
    c.to_digit(10)
        .unwrap_or_else(|| panic!("{} couldn't be converted to base 10 digit", c)) as usize
}

#[allow(dead_code)]
fn print_blocks(blocks: &Vec<Option<usize>>) {
    println!("{}", blocks.iter().map(|block| match block {
        Some(val) => val.to_string(),
        None => ".".to_owned(),
    }).join(""));
}

type FileId = usize;
type Index = usize;
type FileSize = usize;

pub fn part_one(input: &str) -> Option<u64> {
    let mut blocks: Vec<Option<FileId>> = Vec::new();
    let mut is_file = true;
    let mut curr_id = 0usize;
    for c in input.trim().chars() {
        if is_file {
            blocks.extend(repeat_n(Some(curr_id), char_to_digit(c)));
            curr_id += 1;
        } else {
            blocks.extend(repeat_n(None, char_to_digit(c)));
        }
        is_file = !is_file;
    }

    let mut free_idx = 0usize;
    loop {
        //print_blocks(&blocks);
        match blocks.get(free_idx) {
            Some(Some(_)) => (), // do nothing
            None => break,       // we've reached the end of the blocks
            Some(None) => loop {
                if let Some(Some(_)) = blocks.last() {
                    blocks.swap_remove(free_idx);
                    break;
                }
                let _ = blocks.pop();
            },
        }
        free_idx += 1;
    }

    Some(
        blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                Some(int) => Some((i * *int) as u64),
                None => None,
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blocks: Vec<Option<FileId>> = Vec::new();
    let mut file_start_idxs: Vec<(FileSize, Index)> = Vec::new();
    let mut is_file = true;
    let mut curr_id = 0usize;
    for c in input.trim().chars() {
        let block_count: FileSize = char_to_digit(c);
        if is_file {
            file_start_idxs.push((block_count, blocks.len()));
            blocks.extend(repeat_n(Some(curr_id), block_count));
            curr_id += 1;
        } else {
            blocks.extend(repeat_n(None, block_count));
        }
        is_file = !is_file;
    }

    let mut inner_iter: Index;
    let mut inner_lookahead: Index;
    let mut swapped: bool;
    // iterate through each placed file in reverse order
    while let Some((size, idx)) = file_start_idxs.pop() {
        //print_blocks(&blocks);
        inner_iter = 0;
        swapped = false;
        // starting from the front of the file,
        loop {
            // check the contents of the next block
            match blocks.get(inner_iter) {
                Some(Some(_)) => (), // do nothing if there's a file there
                None => break,       // we've reached the end of the blocks, go to next file
                Some(None) => { // free space!
                    inner_lookahead = inner_iter;
                    // starting from this free space, move forward and check if there are enough
                    // free spaces to fit the current file
                    loop {
                        match blocks.get(inner_lookahead) {
                            Some(None) => { // still free space
                                // swap if we have looked far enough ahead to fit the file
                                if (inner_lookahead - inner_iter) == (size - 1) {
                                    (inner_iter..=inner_lookahead).zip(idx..(idx+size)).for_each(|(free_idx, file_idx)| blocks.swap(free_idx, file_idx));
                                    swapped = true;
                                    break
                                }
                            },
                            None => break, // end of blocks, leave
                            Some(Some(_)) => { // hit another file before finding enough space to
                                               // fit current one, advance iterator to this point
                                               // to avoid rechecking what we already checked
                                inner_iter = inner_lookahead;
                                break
                            }
                        }
                        inner_lookahead += 1;
                    }
                    // if a swap occurred, break out of secondary loop
                    if swapped {
                        break
                    }
                }
            }
            inner_iter += 1;
            // if the iterator is past the start point of the current file, there's no space for it
            if inner_iter >= idx {
                break;
            }
        }
    }

    Some(
        blocks
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                Some(int) => Some((i * *int) as u64),
                None => None,
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

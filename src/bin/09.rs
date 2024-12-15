use std::collections::VecDeque;

use rayon::iter::Empty;

advent_of_code::solution!(9);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Block {
    Empty,
    File(u64),
}

impl Block {
    fn is_file(&self) -> bool {
        matches!(self, Block::File(_))
    }

    fn file_id(&self) -> Option<u64> {
        match &self {
            Block::File(id) => Some(*id),
            Block::Empty => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    block: Block,
    size: i64,
}

fn parse(input: &str) -> Vec<Segment> {
    let bytes = input.as_bytes();

    let mut res = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    for b in bytes {
        let Some(val) = b.checked_sub('0' as u8) else {
            break;
        };

        let block = if is_file {
            id += 1;
            Block::File(id - 1)
        } else {
            Block::Empty
        };

        let size = val as i64;
        if size != 0 {
            res.push(Segment { block, size });
        }

        is_file = !is_file
    }

    res
}

fn dump<'a>(segments: impl Iterator<Item = &'a Segment>) {
    for segment in segments {
        let c = match segment.block {
            Block::File(id) => format!("{}", id),
            Block::Empty => ".".to_string(),
        };
        for _ in 0..segment.size {
            print!("{}", c);
        }
    }
}

fn calc_answer(segmented: &[Segment]) -> u64 {
    let mut values: Vec<u64> = Vec::new();
    for segment in segmented {
        let id = match segment.block {
            Block::File(id) => id,
            Block::Empty => 0,
        };
        for _ in 0..segment.size {
            values.push(id);
        }
    }

    values
        .iter()
        .enumerate()
        .map(|(pos, id)| (pos as u64) * id)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut original: VecDeque<_> = parse(input).into();

    let mut segmented: Vec<Segment> = Vec::new();

    while let Some(last) = original.back() {
        if last.size == 0 || !last.block.is_file() {
            original.pop_back();
        } else {
            break;
        }
    }

    while original.len() > 1 {
        let first = original.pop_front().unwrap();

        if first.block.is_file() {
            segmented.push(first);
        } else {
            let last = original.pop_back().unwrap();
            // This must be a file
            assert!(last.block.is_file());

            if last.size >= first.size {
                segmented.push(Segment {
                    block: last.block.clone(),
                    size: first.size,
                });
                original.push_back(Segment {
                    block: last.block,
                    size: last.size - first.size,
                });
            } else {
                segmented.push(Segment {
                    block: last.block.clone(),
                    size: last.size,
                });
                original.push_front(Segment {
                    block: first.block,
                    size: first.size - last.size,
                });
            }
        }

        while let Some(last) = original.back() {
            if last.size == 0 || !last.block.is_file() {
                original.pop_back();
            } else {
                break;
            }
        }
    }

    if let Some(last) = original.front() {
        if last.block.is_file() {
            segmented.push(last.clone());
        }
    }

    Some(calc_answer(&segmented))
}

fn normalize_part_two(original: Vec<Segment>) -> Vec<Segment> {
    let mut ans: Vec<Segment> = Vec::new();

    for segment in original {
        if ans.is_empty() || segment.block.is_file() || ans.last().unwrap().block.is_file() {
            ans.push(segment);
        } else {
            let last = ans.last_mut().unwrap();
            last.size += segment.size;
        }
    }
    ans
}


fn run_for_id(input: Vec<Segment>, id: u64) -> Vec<Segment> {
    let mut original: VecDeque<_> = normalize_part_two(input).into();

    let mut segmented: Vec<Segment> = Vec::new();

    let to_add = {
        let mut to_add: Vec<Segment> = Vec::new();

        while let Some(back) = original.pop_back() {
            if back.block.file_id() == Some(id) {
                original.push_back(back);
                break;
            } else {
                to_add.push(back);
            }
        }

        to_add.reverse();
        to_add
    };

    let mut target = original.pop_back().unwrap();


    while !original.is_empty() {
        let first = original.pop_front().unwrap();

        if first.block.is_file() || first.size < target.size {
            segmented.push(first);
        } else {
            segmented.push(target.clone());
            target.block = Block::Empty;
            if first.size != target.size {
                segmented.push(Segment{
                    block: Block::Empty,
                    size: first.size - target.size,
                });
            }
            break;
        }
    }
    segmented.extend(original.iter());
    segmented.push(target);
    segmented.extend_from_slice(&to_add);
    segmented
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut segments = parse(input);
    let max_id =
        segments.iter()
        .map(|s| s.block.file_id().unwrap_or(0))
        .max().unwrap();

    for id in (0..max_id+1).rev() {
        segments = run_for_id(segments, id);
    }

    Some(calc_answer(&segments))
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
    fn test_part_one_full() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6398608069280));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

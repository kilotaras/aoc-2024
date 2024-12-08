use std::collections::HashSet;

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq)]
struct Puzzle {
    target: u64,
    operands: Vec<u64>,
}

impl From<&str> for Puzzle {
    fn from(input: &str) -> Self {
        let mut parts = input.split(":");
        let target = parts.next().unwrap().parse().unwrap();

        let operands = parts
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Puzzle { target, operands }
    }
}

fn concat(x: u64, y: u64) -> u64 {
    let mut y1 = y;
    let mut x = x;
    while y1 > 0 {
        x *= 10;
        y1 /= 10;
    }

    x + y
}


fn log10ceil(x: u64) -> u64 {
    let mut ans = 1;
    while ans <= x {
        ans *= 10;
    }

    ans
}


fn is_achieavable_part_two_fast(target: u64, operands: &[u64]) -> bool {
    if operands.is_empty() {
        panic!("SHould not happen");
    }

    if target <= 0 {
        return false;
    }
    if operands.len() == 1 {
        return  target == operands[0];
    }


    let rest = &operands[0..operands.len()-1];
    let last = *operands.last().unwrap();


    // try multiply

    if target%last == 0 {
        let new_target = target/last;
        if is_achieavable_part_two_fast(new_target, rest) {
            return true;
        }
    }

    // Try concat

    let pow10 = log10ceil(last);
    if target%pow10 == last {
        let new_target = target/pow10;
        if is_achieavable_part_two_fast(new_target, rest) {
            return true;
        }
    }

    if target > last {
        is_achieavable_part_two_fast(target-last, rest)
    } else {
        false
    }
}

impl Puzzle {
    fn is_achieavable_part_one(&self) -> bool {
        let mut frontier = HashSet::new();
        let mut operands = self.operands.iter().copied();
        frontier.insert(operands.next().unwrap());

        while let Some(v) = operands.next() {
            let mut new_frontier = HashSet::new();
            for f in frontier {
                if f <= self.target {
                    new_frontier.insert(f + v);
                    new_frontier.insert(f * v);
                }
            }

            frontier = new_frontier;
        }

        frontier.contains(&self.target)
    }

    fn is_achieavable_part_two(&self) -> bool {
        is_achieavable_part_two_fast(self.target, &self.operands)
    }
}

fn parse(input: &str) -> Vec<Puzzle> {
    input.lines()
        .map(|l| l.into())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzles = parse(input);

    let ans = puzzles
        .into_iter()
        .filter(|p| p.is_achieavable_part_one())
        .map(|p| p.target)
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let puzzles = parse(input);

    let ans = puzzles
        .into_iter()
        .filter(|p| p.is_achieavable_part_two())
        .map(|p| p.target)
        .sum();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_one() {
        let puzzle: Puzzle = "3267: 81 40 27".into();
        assert_eq!(puzzle, Puzzle {target: 3267, operands: vec![81, 40, 27]});
    }

    #[test]
    fn test_achievable() {
        let puzzle: Puzzle = "3267: 81 40 27".into();
        assert!(puzzle.is_achieavable_part_one());

        let puzzle: Puzzle = "161011: 16 10 13".into();
        assert!(!puzzle.is_achieavable_part_one());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

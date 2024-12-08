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

fn log10ceil(x: u64) -> u64 {
    let mut ans = 1;
    while ans <= x {
        ans *= 10;
    }

    ans
}

trait ReverseOp {
    fn reverse(&self, target: u64) -> Vec<u64>;
    fn value(&self) -> u64;
}

struct OpPart1(u64);

impl ReverseOp for OpPart1 {
    fn reverse(&self, target: u64) -> Vec<u64> {
        let mut ans = Vec::new();

        if target%self.0 == 0 {
            ans.push(target/self.0);
        }

        if target>self.0 {
            ans.push(target-self.0);
        }

        ans
    }

    fn value(&self) -> u64 {
        self.0
    }

}

struct OpPart2 {
    operand: u64,
    pow10: u64
}

impl OpPart2 {
    fn new(v: u64) -> Self {
        OpPart2 {
            operand: v,
            pow10: log10ceil(v)
        }
    }
}

impl ReverseOp for OpPart2 {
    fn reverse(&self, target: u64) -> Vec<u64> {
        let mut ans = Vec::new();

        if target%self.operand == 0 {
            ans.push(target/self.operand);
        }

        if target>self.operand {
            ans.push(target-self.operand);
        }

        if target%self.pow10 == self.operand {
            ans.push(target/self.pow10);
        }

        ans
    }

    fn value(&self) -> u64 {
        self.operand
    }
}


fn is_achieavable_fast(target: u64, operands: &[impl ReverseOp]) -> bool {
    if operands.is_empty() {
        panic!("SHould not happen");
    }

    if target <= 0 {
        return false;
    }
    if operands.len() == 1 {
        return  target == operands[0].value();
    }


    let rest = &operands[0..operands.len()-1];
    let last = operands.last().unwrap();

    let next_targets = last.reverse(target);

    next_targets.iter()
        .any(|p| is_achieavable_fast(*p, &rest))
}

impl Puzzle {
    fn is_achieavable_part_one(&self) -> bool {
        let operands =
            self.operands
                .iter()
                .map(|v| OpPart1(*v))
                .collect::<Vec<_>>();
        is_achieavable_fast(self.target, &operands)
    }

    fn is_achieavable_part_two(&self) -> bool {
        let operands =
            self.operands
                .iter()
                .map(|v| OpPart2::new(*v))
                .collect::<Vec<_>>();
        is_achieavable_fast(self.target, &operands)
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

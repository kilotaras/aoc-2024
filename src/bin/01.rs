advent_of_code::solution!(1);


pub fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_ascii_whitespace();
        let l = parts.next().unwrap();
        let r = parts.next_back().unwrap();
        left.push(l.parse().unwrap());
        right.push(r.parse().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    Some(left.iter().zip(right).map(|(l,r)| (l-r).abs() as u32).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let (left, right) = parse(input);

    Some(left.into_iter()
        .map(|l| l * (right.iter().filter(|r| **r == l).count() as i32))
        .sum())
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

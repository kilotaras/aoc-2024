use std::collections::HashMap;

advent_of_code::solution!(11);

fn split(v: u64) -> Option<(u64, u64)> {
    let mut pow10 = 10;
    let mut cnt = 1;
    while pow10 <= v {
        pow10 *= 10;
        cnt += 1
    }

    if cnt % 2 == 1 {
        None
    } else {
        let mut pow10 = 1;
        for _ in 0..cnt / 2 {
            pow10 *= 10;
        }
        Some((v / pow10, v % pow10))
    }
}


fn solve(v: u64, time: u8, dp: &mut HashMap<(u64, u8), u64>) -> u64 {
    if time == 0 {
        return 1;
    }

    if v == 0 {
        return solve(1, time - 1, dp)
    }

    if dp.contains_key(&(v, time)) {
        return *dp.get(&(v, time)).unwrap();
    }

    let res = if let Some((l, r)) = split(v) {
        solve(l, time - 1, dp) + solve(r, time - 1, dp)
    } else {
        solve(2024 * v, time - 1, dp)
    };

    dp.insert((v, time), res);

    res
}

fn sol_many(nums: &[u64], time: u8) -> u64 {
    let mut dp = HashMap::new();
    nums.iter().map(|v| solve(*v, time, &mut dp)).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let nums: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Some(sol_many(&nums, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Some(sol_many(&nums, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split(123), None);
        assert_eq!(split(123023), Some((123, 23)));
    }

    #[test]
    fn test_samples() {
        assert_eq!(sol_many(&[125, 17], 6), 22);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}

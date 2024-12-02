use std::ops::{Range, RangeInclusive};

advent_of_code::solution!(2);

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|p| p.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(arr: &[i32]) -> bool {
    let mut small_diff = true;
    let mut dec = true;
    let mut inc = true;

    for window in arr.windows(2) {
        if let [l, r] = window {
            if l <= r {
                dec = false;
            }

            if l >= r {
                inc = false;
            }

            if (r - l).abs() > 3 {
                small_diff = false;
            }
        }
    }

    let result = small_diff && (dec || inc);
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let levers = parse(input);
    Some(levers.into_iter().filter(|v| is_valid(v)).count() as u32)
}

fn can_make_valid(arr: &[i32], diff_range: RangeInclusive<i32>) -> bool {
    let mut dp = vec![[false, false]; arr.len()];
    dp[0][0] = true;
    dp[0][1] = true;
    dp[1][1] = true;

    if diff_range.contains(&(arr[1] - arr[0])) {
        dp[1][0] = true;
    }

    for p in 2..arr.len() {
        let d1 = arr[p] - arr[p-1];
        let d2 = arr[p] - arr[p-2];

        if diff_range.contains(&d1) {
            dp[p][0] = dp[p-1][0];
            dp[p][1] = dp[p-1][1];
        }

        if diff_range.contains(&d2) {
            dp[p][1] |= dp[p-2][0];
        }

        dp[p][1] |= dp[p][0];
    }

    dp[arr.len()-1][1] || dp[arr.len()-2][0]
}

#[test]
fn test_can_valid() {
    let arr = [10, 12, 14, 17, 20, 23, 26, 30];
    assert!(can_make_valid(&arr, 1..=3));
}


fn can_make_safe(arr: &[i32]) -> bool {
    can_make_valid(arr, 1..=3) || can_make_valid(arr, -3..=-1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let levers = parse(input);
    Some(levers.into_iter().filter(|v| can_make_safe(v)).count() as u32)
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
        assert_eq!(result, Some(4));
    }
}

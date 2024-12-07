advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut sum = 0;
    for mul_op in re.find_iter(input) {
        let parts = mul_op
            .as_str()
            .split(|c| c == '(' || c == ')' || c == ',')
            .collect::<Vec<_>>();

        if let [_, l, r, ..] = parts[..] {
            let l: u32 = l.parse().unwrap();
            let r: u32 = r.parse().unwrap();
            sum += (l * r);
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for group in re.find_iter(input) {
        let s = group.as_str();
        match s {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            mul_op if enabled => {
                let parts = mul_op
                    .split(|c| c == '(' || c == ')' || c == ',')
                    .collect::<Vec<_>>();

                if let [_, l, r, ..] = parts[..] {
                    let l: u32 = l.parse().unwrap();
                    let r: u32 = r.parse().unwrap();
                    sum += l * r;
                }
            }
            _ => {}
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}

use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(PartialEq, Eq, Debug)]
struct Rule {
    from: u32,
    to: u32,
}

fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let rules = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split('|');
            let from = parts.next().unwrap().parse().unwrap();
            let to = parts.next().unwrap().parse().unwrap();
            Rule { from, to }
        })
        .collect();

    let pages = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}

fn is_valid(rules: &Vec<Rule>, manual: &Vec<u32>) -> bool {
    for i in 0..manual.len() {
        for j in 0..i {
            for rule in rules {
                if rule.from == manual[i] && rule.to == manual[j] {
                    return false;
                }
            }
        }
    }
    return true;
}
pub fn part_one(input: &str) -> Option<u32> {
    let (rules, pages) = parse(input);
    let valid = pages
        .into_iter()
        .filter(|v| is_valid(&rules, v))
        .collect::<Vec<_>>();

    let ans = valid
        .into_iter()
        .map(|v| v[v.len() / 2])
        .sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let (rules, pages) = parse("1|2\n3|4\n\n1,2,3\n3,4");
        assert_eq!(
            rules,
            vec![Rule { from: 1, to: 2 }, Rule { from: 3, to: 4 }]
        );
        assert_eq!(pages, vec![vec![1, 2, 3], vec![3, 4]]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

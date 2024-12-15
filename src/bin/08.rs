use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);


#[derive(Debug, Eq, PartialEq)]
struct Field {
    rows: usize,
    cols: usize,
    beacons: HashMap<char, Vec<(i64, i64)>>
}


fn parse(input: &str) -> Field {
    let lines = input.lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let rows = lines.len();
    let cols = lines[0].len();


    let mut beacons: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            if lines[i][j] != '.' {
                beacons.entry( lines[i][j]).or_default().push((i as i64,j as i64));
            }
        }
    }

    Field {
        rows, cols, beacons
    }
}


impl Field {
    fn gen_antinodes(&self)  -> Vec<(i64, i64)> {
        let mut ans: HashSet<(i64, i64)> = HashSet::new();
        for (_, positions) in &self.beacons {
            for (p1, p2) in positions.iter()
                .cartesian_product(positions.iter())
                .filter(|(p1, p2)| p1 != p2) {
                    let dx = p2.0 - p1.0;
                    let dy = p2.1 - p1.1;
                    ans.insert((p1.0 - dx, p1.1 - dy));
                }
        }

        ans.into_iter().collect()
    }

    fn contains(&self, (x, y): (i64, i64)) -> bool {
        let range_rows = 0..(self.rows as i64);
        let range_cols = 0..(self.cols as i64);

        range_rows.contains(&x) && range_cols.contains(&y)
    }

    fn gen_antinodes_part_two(&self)  -> Vec<(i64, i64)> {
        let mut ans: HashSet<(i64, i64)> = HashSet::new();
        for (_, positions) in &self.beacons {
            for (p1, p2) in positions.iter()
                .cartesian_product(positions.iter())
                .filter(|(p1, p2)| p1 != p2) {
                    let dx = p2.0 - p1.0;
                    let dy = p2.1 - p1.1;
                    for i in -70..70 {
                        ans.insert((p1.0 - i*dx, p1.1 - i*dy));
                    }
                }
        }

        ans.into_iter().collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse(input);

    let antinodes = field.gen_antinodes();

    let ans =
        antinodes.iter()
            .filter(|p| field.contains(**p))
            .count();

    Some(ans as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = parse(input);

    let antinodes = field.gen_antinodes_part_two();

    let ans =
        antinodes.iter()
            .filter(|p| field.contains(**p))
            .count();

    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

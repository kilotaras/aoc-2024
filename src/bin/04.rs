advent_of_code::solution!(4);

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

trait Field2d {
    fn get_at_pos(&self, row: i32, col: i32) -> Option<char>;
}

impl Field2d for Vec<Vec<char>> {
    fn get_at_pos(&self, row: i32, col: i32) -> Option<char> {
        if (row < 0) {
            return None;
        }

        if (col < 0) {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        self.get(row)?
            .get(col)
            .cloned()
    }
}

fn cnt_xmas(field: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    fn equal(field: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32, needle: &str) -> bool {
        if needle.is_empty() {
            true
        } else {
            let mut chars = needle.chars();
            let fchar = chars.next();
            if field.get_at_pos(x, y) == fchar {
                equal(field, x+dx, y+dy, dx, dy, chars.as_str())
            } else {
                false
            }
        }
    }

    let x = x as i32;
    let y = y as i32;

    let mut cnt = 0;

    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if equal(field, x, y, dx, dy, "XMAS") {
                cnt += 1
            }
        }
    }

    cnt
}

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse(input);
    let mut ans = 0;
    for row in 0..field.len() {
        for col in 0..field[0].len() {
            ans += cnt_xmas(&field, row, col);
        }
    }

    Some(ans)
}

fn is_cross_mas(field: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    fn inner(field: &Vec<Vec<char>>, x: i32, y: i32) -> Option<()> {
        if field.get_at_pos(x, y)? != 'A' {
            return None
        }

        {
            let f1 = field.get_at_pos(x-1, y-1)?;
            let f2 = field.get_at_pos(x+1, y+1)?;

            if !((f1 == 'M' && f2 == 'S') || (f1 == 'S' && f2 == 'M')) {
                return None;
            }
        }

        {
            let f1 = field.get_at_pos(x-1, y+1)?;
            let f2 = field.get_at_pos(x+1, y-1)?;

            if !((f1 == 'M' && f2 == 'S') || (f1 == 'S' && f2 == 'M')) {
                return None;
            }
        }

        Some(())
    }

    inner(field, x as i32, y as i32).is_some()
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = parse(input);
    let mut ans = 0;
    for row in 0..field.len() {
        for col in 0..field[0].len() {
            if is_cross_mas(&field, row, col) {
                ans += 1;
            }
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

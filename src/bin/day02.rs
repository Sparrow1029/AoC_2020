use aoc_2020::get_puzzle_input_string;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Password {
    start: usize,
    end: usize,
    char: char,
    pwd: Vec<char>,
}

impl From<&str> for Password {
    fn from(value: &str) -> Self {
        let (rest, (start, end)) = get_range(value).expect("nom error parsing range");
        let check_char = rest[..1].as_bytes()[0] as char;
        let pwd: Vec<char> = rest[3..].as_bytes().iter().map(|b| *b as char).collect();
        Password {
            start,
            end,
            char: check_char,
            pwd,
        }
    }
}

impl Password {
    fn valid_part1(&self) -> bool {
        let pwd = self.pwd.clone();
        let count: usize = pwd.into_iter().filter(|c| *c == self.char).count();
        (self.start..=self.end).contains(&count)
    }

    fn valid_part2(&self) -> bool {
        let (char1, char2) = (self.pwd[self.start - 1], self.pwd[self.end - 1]);
        (self.char == char1 || self.char == char2) && (char1 != char2)
    }
}

fn get_range(input: &str) -> IResult<&str, (usize, usize)> {
    let (rest, (start, _, end, _)) = tuple((digit1, tag("-"), digit1, space1))(input)?;
    Ok((
        rest,
        (
            start.parse::<usize>().unwrap(),
            end.parse::<usize>().unwrap(),
        ),
    ))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(|p| p.valid_part1())
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(Password::from)
        .filter(|p| p.valid_part2())
        .count()
}

fn main() {
    let input = get_puzzle_input_string(2).expect("I/O Error");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
    use crate::Password;

    const INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn test_password() {
        let pwds: Vec<Password> = INPUT.lines().map(Password::from).collect();
        assert!(!(pwds[1].valid_part1()));
        assert!(!(pwds[2].valid_part2()));
    }
}

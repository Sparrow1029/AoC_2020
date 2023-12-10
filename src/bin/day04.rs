use std::{collections::HashMap, ops::RangeInclusive};

use aoc_2020::get_puzzle_input_string;

type Passport<'a> = HashMap<&'a str, &'a str>;

const REQUIRED_FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

fn parse_passport(input: &str) -> Passport {
    let items: Vec<&str> = input.split_whitespace().collect();
    let pairs: Vec<(&str, &str)> = items
        .into_iter()
        .map(|item| {
            let pair: Vec<&str> = item.split(':').collect();
            (pair[0], pair[1])
        })
        .collect();
    HashMap::from_iter(pairs)
}

/// Check number of fields is correct, and that all required fields are present
fn valid(passport: &Passport, num_fields: usize, must_be_present: &[&str]) -> bool {
    if passport.len() >= num_fields {
        return must_be_present.iter().all(|k| passport.contains_key(*k));
    }
    false
}

/// Assert `year_str` is contained within `rng` (inclusive)
fn validate_year(year_str: &str, rng: RangeInclusive<usize>) -> bool {
    let v = year_str.parse::<usize>().unwrap_or_default();
    year_str.len() == 4 && rng.contains(&v)
}

/// Validate a passport field based on arbitrary criteria
fn validate_field(field: (&str, &str)) -> bool {
    let (k, v) = field;
    match k {
        "byr" => validate_year(v, 1920..=2002),
        "iyr" => validate_year(v, 2010..=2020),
        "eyr" => validate_year(v, 2020..=2030),
        "hgt" => {
            let (num, meas) = v.split_at(v.len() - 2);
            if num.chars().all(|c| c.is_ascii_digit()) && ["cm", "in"].contains(&meas) {
                let num = num.parse::<usize>().unwrap_or_default();
                match meas {
                    "cm" => (150..=193).contains(&num),
                    "in" => (59..=76).contains(&num),
                    _ => unreachable!(),
                }
            } else {
                false
            }
        }
        "hcl" => {
            let hex = v.as_bytes();
            hex[0] == b'#' && hex[1..].iter().all(|b| (*b as char).is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => false,
    }
}

/// Iterate all fields of passport and determine if they are valid according to rules
fn validate_extra(passport: &Passport) -> bool {
    passport.iter().all(|(k, v)| validate_field((*k, *v)))
}

fn part_1(passports: &[Passport], must_be_present: &[&str]) -> usize {
    passports
        .iter()
        .filter(|p| valid(p, must_be_present.len(), must_be_present))
        .count()
}

fn part_2(passports: &[Passport], must_be_present: &[&str]) -> usize {
    passports
        .iter()
        .filter(|p| valid(p, must_be_present.len(), must_be_present))
        .filter(|p| validate_extra(p))
        .count()
}

fn main() {
    let input = get_puzzle_input_string(4).expect("I/O error");

    let passports: Vec<Passport> = input.split("\n\n").map(parse_passport).collect();
    println!("Part 1: {}", part_1(&passports, &REQUIRED_FIELDS));
    println!("Part 2: {}", part_2(&passports, &REQUIRED_FIELDS));
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const PART2_SAMPLE: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    const SINGLE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

    #[test]
    fn test_parse() {
        let passport = parse_passport(SINGLE);
        let passport_vec = vec![
            ("ecl", "gry"),
            ("pid", "860033327"),
            ("eyr", "2020"),
            ("hcl", "#fffffd"),
            ("byr", "1937"),
            ("iyr", "2017"),
            ("cid", "147"),
            ("hgt", "183cm"),
        ];
        assert_eq!(passport, HashMap::from_iter(passport_vec));
    }

    #[test]
    fn test_valid_part_1() {
        let passports: Vec<Passport> = SAMPLE.split("\n\n").map(parse_passport).collect();
        assert_eq!(part_1(&passports, &REQUIRED_FIELDS), 2);
    }

    #[test]
    fn test_extra_valid() {
        let invalid = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";
        let invalid_passport = parse_passport(invalid);
        assert!(validate_extra(&invalid_passport));
        //         let invalid = "\
        // pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
        // hcl:#623a2f";
    }

    #[test]
    fn test_valid_part2() {
        let passports: Vec<Passport> = PART2_SAMPLE.split("\n\n").map(parse_passport).collect();
        assert_eq!(part_2(&passports, &REQUIRED_FIELDS), 4);
    }
}

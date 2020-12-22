/*
    Advent of Code 2020
    Caleb Stanford
    Day 4 Solution
    2020-12-06
*/

#![allow(dead_code)]

use aoc2020::util::file_to_vec;

use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

/*
    Part 1 datatype: unvalidated passport data
*/

fn get_fields(data: &HashMap<String, String>) -> HashSet<String> {
    data.keys().map(|s| s.to_owned()).collect()
}
fn get_field_or_error(
    data: &HashMap<String, String>,
    field: &str,
) -> Result<String, String> {
    data.get(field)
        .ok_or(format!("field not found: {}", field))
        .map(|s| s.to_owned())
}
fn get_field_or_none(
    data: &HashMap<String, String>,
    field: &str,
) -> Option<String> {
    data.get(field).map(|s| s.to_owned())
}

struct PassportRaw {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}
impl TryFrom<HashMap<String, String>> for PassportRaw {
    type Error = String;
    fn try_from(data: HashMap<String, String>) -> Result<Self, Self::Error> {
        // Note: this doesn't check that data *only* has required fields;
        // it just checks that data has at least the required fields.
        let byr = get_field_or_error(&data, "byr")?;
        let iyr = get_field_or_error(&data, "iyr")?;
        let eyr = get_field_or_error(&data, "eyr")?;
        let hgt = get_field_or_error(&data, "hgt")?;
        let hcl = get_field_or_error(&data, "hcl")?;
        let ecl = get_field_or_error(&data, "ecl")?;
        let pid = get_field_or_error(&data, "pid")?;
        let cid = get_field_or_none(&data, "cid");
        Ok(Self { byr, iyr, eyr, hgt, hcl, ecl, pid, cid })
    }
}

/*
    Part 2 datatype: validated passport data
*/

// Reusable validators for int and char ranges
fn validate_int_range(
    n: usize,
    low: usize,
    high: usize,
) -> Result<usize, String> {
    if n >= low && n <= high {
        Ok(n)
    } else {
        Err(format!(
            "Invalid int: {} (should be between {} and {})",
            n, low, high,
        ))
    }
}
fn validate_char_range(
    ch: char,
    low: char,
    high: char,
) -> Result<char, String> {
    if ch >= low && ch <= high {
        Ok(ch)
    } else {
        Err(format!(
            "Invalid char: {} (should be between {} and {})",
            ch, low, high,
        ))
    }
}

// A few specific custom validators
fn validate_date(
    date_str: &str,
    low: usize,
    high: usize,
) -> Result<usize, String> {
    let parsed = date_str.parse().or_else(|err| {
        Err(format!("could not parse as int: {} ({:?})", date_str, err))
    })?;
    validate_int_range(parsed, low, high)
}
fn validate_height(hgt: &str) -> Result<(usize, String), String> {
    let n = hgt.chars().count();
    let first_part: String = hgt.chars().take(n - 2).collect();
    let second_part: String = hgt.chars().skip(n - 2).collect();
    assert!(second_part.chars().count() == 2);

    let parsed = first_part.parse().or_else(|err| {
        Err(format!("could not parse as int: {} ({:?})", first_part, err))
    })?;
    let validated = match second_part.as_ref() {
        "cm" => validate_int_range(parsed, 150, 193),
        "in" => validate_int_range(parsed, 59, 76),
        other => Err(format!("not a valid length unit: {}", other)),
    }?;
    Ok((validated, second_part))
}
fn validate_hair_color(color: &str) -> Result<String, String> {
    if color.chars().count() != 7 {
        return Err(format!(
            "Not a valid color (should be 7 digits): {}",
            color
        ));
    }
    let mut first = true;
    for ch in color.chars() {
        if first {
            validate_char_range(ch, '#', '#')?;
            first = false;
        } else {
            validate_char_range(ch, '0', '9')
                .or_else(|_err| validate_char_range(ch, 'a', 'f'))?;
        }
    }
    Ok(color.to_owned())
}
fn validate_eye_color(color: &str) -> Result<String, String> {
    match color {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
            Ok(color.to_owned())
        }
        _ => Err(format!("invalid eye color: {}", color)),
    }
}
fn validate_pid(pid: &str) -> Result<String, String> {
    if pid.chars().count() != 9 {
        return Err(format!("Not a valid PID (should be 9 digits): {}", pid));
    }
    for ch in pid.chars() {
        validate_char_range(ch, '0', '9')?;
    }
    Ok(pid.to_owned())
}

struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: (usize, String),
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}
impl TryFrom<PassportRaw> for Passport {
    type Error = String;
    fn try_from(passport_raw: PassportRaw) -> Result<Self, Self::Error> {
        let byr = validate_date(&passport_raw.byr, 1920, 2002)?;
        let iyr = validate_date(&passport_raw.iyr, 2010, 2020)?;
        let eyr = validate_date(&passport_raw.eyr, 2020, 2030)?;
        let hgt = validate_height(&passport_raw.hgt)?;
        let hcl = validate_hair_color(&passport_raw.hcl)?;
        let ecl = validate_eye_color(&passport_raw.ecl)?;
        let pid = validate_pid(&passport_raw.pid)?;
        let cid = passport_raw.cid;
        Ok(Self { byr, iyr, eyr, hgt, hcl, ecl, pid, cid })
    }
}

/*
    Resulting solutions to part 1 and part 2
*/

fn solve_part1(data: Vec<HashMap<String, String>>) -> usize {
    data.into_iter().map(PassportRaw::try_from).filter(|x| x.is_ok()).count()
}

fn solve_part2(data: Vec<HashMap<String, String>>) -> usize {
    data.into_iter()
        .map(PassportRaw::try_from)
        .filter(|x| x.is_ok())
        .map(|x| Passport::try_from(x.unwrap()))
        .filter(|x| x.is_ok())
        .count()
}

fn main() {
    /* Parse Input */
    let input_tokens: Vec<String> = file_to_vec("input/day04.txt")
        .iter()
        .flat_map(|line| line.split(' '))
        .map(|s| s.to_owned())
        .collect();
    let mut input = vec![HashMap::new()];
    let mut last_index = 0;
    for token in input_tokens {
        if token == "" {
            input.push(HashMap::new());
            last_index += 1;
        } else {
            let parts: Vec<&str> = token.split(':').collect();
            assert!(parts.len() == 2);
            let key = parts[0].to_owned();
            let value = parts[1].to_owned();
            assert!(!input[last_index].contains_key(&key));
            input[last_index].insert(key, value);
        }
    }
    // println!("Input: {:?}", input);
    /* Solve and Output Answer */
    println!("Part 1 Answer: {}", solve_part1(input.clone()));
    println!("Part 2 Answer: {}", solve_part2(input));
}

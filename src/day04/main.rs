use anyhow::{bail, Context};
use nom::{
    alt, call, char,
    character::streaming::{alphanumeric1, digit1},
    complete, do_parse, map, map_res, named, opt, preceded, separated_list0, separated_list1, tag,
    take_while1, value,
};
use std::collections::{HashMap, HashSet};

const INPUT: &[u8] = include_bytes!("input.txt");
const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Height {
    Cm(u32),
    In(u32),
}

named!(
    parse_newline<()>,
    value!((), preceded!(opt!(char!('\r')), char!('\n')))
);

named!(
    parse_field<(&str, &str)>,
    do_parse!(
        key: map_res!(call!(alphanumeric1), std::str::from_utf8)
            >> char!(':')
            >> value:
                map_res!(
                    take_while1!(|c: u8| !c.is_ascii_whitespace()),
                    std::str::from_utf8
                )
            >> ((key, value))
    )
);

named!(
    parse_passport<HashMap<&str, &str>>,
    do_parse!(
        fields:
            separated_list1!(
                alt!(value!((), char!(' ')) | parse_newline),
                complete!(parse_field)
            )
            >> parse_newline
            >> (fields.into_iter().collect())
    )
);

named!(
    parse_passports<Vec<HashMap<&str, &str>>>,
    separated_list0!(complete!(parse_newline), complete!(parse_passport))
);

named!(
    parse_height<&str, Height>,
    do_parse!(
        value: map_res!(call!(digit1), |s: &str| s.parse())
            >> height: alt!(
                map!(tag!("cm"), |_| Height::Cm(value))
                | map!(tag!("in"), |_| Height::In(value))
            )
            >> (height)
    )
);

fn part1(input: &'static [u8]) -> anyhow::Result<usize> {
    let (remainder, passports) = parse_passports(input).context("failed to parse input")?;
    if !remainder.is_empty() {
        bail!(
            "input was not fully parsed (remainder: {:?})",
            std::str::from_utf8(remainder)
        );
    }

    let valid = passports
        .into_iter()
        .filter(|passport| {
            let mut required_fields: HashSet<_> = REQUIRED_FIELDS.iter().copied().collect();
            passport.keys().for_each(|key| {
                required_fields.remove(key);
            });
            required_fields.is_empty()
        })
        .count();

    Ok(valid)
}

fn part2(input: &'static [u8]) -> anyhow::Result<usize> {
    let (remainder, passports) = parse_passports(input).context("failed to parse input")?;
    if !remainder.is_empty() {
        bail!(
            "input was not fully parsed (remainder: {:?})",
            std::str::from_utf8(remainder)
        );
    }

    let eye_colors: HashSet<_> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .copied()
        .collect();
    let valid = passports
        .into_iter()
        .filter(|passport| {
            let mut required_fields: HashSet<&str> = REQUIRED_FIELDS.iter().copied().collect();
            passport
                .iter()
                .filter(|(&key, &value)| match key {
                    "byr" => {
                        value.len() == 4
                            && value
                                .parse()
                                .ok()
                                .filter(|&n: &u32| n >= 1920 && n <= 2002)
                                .is_some()
                    }
                    "iyr" => {
                        value.len() == 4
                            && value
                                .parse()
                                .ok()
                                .filter(|&n: &u32| n >= 2010 && n <= 2020)
                                .is_some()
                    }
                    "eyr" => {
                        value.len() == 4
                            && value
                                .parse()
                                .ok()
                                .filter(|&n: &u32| n >= 2020 && n <= 2030)
                                .is_some()
                    }
                    "hgt" => parse_height(value)
                        .ok()
                        .filter(|(remainder, height)| {
                            remainder.is_empty()
                                && match height {
                                    Height::Cm(value) => *value >= 150 && *value <= 193,
                                    Height::In(value) => *value >= 59 && *value <= 76,
                                }
                        })
                        .is_some(),
                    "hcl" => {
                        value.len() == 7
                            && value.as_bytes()[0] == b'#'
                            && value.as_bytes()[1..].iter().all(|c| c.is_ascii_hexdigit())
                    }
                    "ecl" => eye_colors.contains(value),
                    "pid" => value.len() == 9 && value.bytes().all(|b| b.is_ascii_digit()),
                    _ => true,
                })
                .for_each(|(key, _)| {
                    required_fields.remove(key);
                });
            required_fields.is_empty()
        })
        .count();

    Ok(valid)
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

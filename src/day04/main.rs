use anyhow::Context;
use nom::{
    alt, call, char,
    character::streaming::{alphanumeric1, digit1},
    complete, do_parse, exact, map, map_res, named, named_args, opt, preceded, separated_list0,
    separated_list1, tag, take, take_while1, value,
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
    exact!(separated_list0!(
        complete!(parse_newline),
        complete!(parse_passport)
    ))
);

named_args!(
    parse_fixed_width_num(digits: usize)<&str, u32>,
    exact!(map_res!(take!(digits), |s: &str| s.parse()))
);

named!(
    parse_height<&str, Height>,
    exact!(
        do_parse!(
            value: map_res!(call!(digit1), |s: &str| s.parse())
                >> height: alt!(
                    map!(tag!("cm"), |_| Height::Cm(value))
                    | map!(tag!("in"), |_| Height::In(value))
                )
                >> (height)
        )
    )
);

fn part1(input: &'static [u8]) -> anyhow::Result<usize> {
    let (_, passports) = parse_passports(input).context("failed to parse input")?;
    let valid = passports
        .into_iter()
        .filter(|passport| {
            passport
                .keys()
                .fold(
                    REQUIRED_FIELDS.iter().copied().collect(),
                    |mut fields: HashSet<_>, key| {
                        fields.remove(key);
                        fields
                    },
                )
                .is_empty()
        })
        .count();

    Ok(valid)
}

fn part2(input: &'static [u8]) -> anyhow::Result<usize> {
    let (_, passports) = parse_passports(input).context("failed to parse input")?;
    let eye_colors: HashSet<_> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .copied()
        .collect();

    let valid = passports
        .into_iter()
        .filter(|passport| {
            passport
                .iter()
                .filter(|(&key, &value)| match key {
                    "byr" => parse_fixed_width_num(value, 4)
                        .ok()
                        .filter(|(_, n)| (1920..=2002).contains(n))
                        .is_some(),
                    "iyr" => parse_fixed_width_num(value, 4)
                        .ok()
                        .filter(|(_, n)| (2010..=2020).contains(n))
                        .is_some(),
                    "eyr" => parse_fixed_width_num(value, 4)
                        .ok()
                        .filter(|(_, n)| (2020..=2030).contains(n))
                        .is_some(),
                    "hgt" => parse_height(value)
                        .ok()
                        .filter(|(_, height)| match height {
                            Height::Cm(value) => (150..=193).contains(value),
                            Height::In(value) => (59..=76).contains(value),
                        })
                        .is_some(),
                    "hcl" => {
                        value.len() == 7
                            && value.as_bytes()[0] == b'#'
                            && value.as_bytes()[1..].iter().all(|b| b.is_ascii_hexdigit())
                    }
                    "ecl" => eye_colors.contains(value),
                    "pid" => parse_fixed_width_num(value, 9).is_ok(),
                    _ => true,
                })
                .fold(
                    REQUIRED_FIELDS.iter().copied().collect(),
                    |mut fields: HashSet<_>, (key, _)| {
                        fields.remove(key);
                        fields
                    },
                )
                .is_empty()
        })
        .count();

    Ok(valid)
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

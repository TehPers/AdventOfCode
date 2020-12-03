use anyhow::{bail, Context};
use nom::{
    call, char,
    character::streaming::{anychar, digit1, line_ending},
    complete, do_parse, many0, map_res, named, opt, take_while1,
};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone)]
struct Policy {
    pub character: char,
    pub bounds: (usize, usize),
}

named!(
    parse_number<usize>,
    map_res!(map_res!(call!(digit1), std::str::from_utf8), |s| {
        usize::from_str_radix(s, 10)
    })
);

named!(
    parse_policy<Policy>,
    do_parse!(
        lower: parse_number
            >> char!('-')
            >> upper: parse_number
            >> char!(' ')
            >> character: call!(anychar)
            >> (Policy {
                character,
                bounds: (lower, upper)
            })
    )
);

named!(
    parse_line<(Policy, &str)>,
    complete!(do_parse!(
        policy: parse_policy
            >> char!(':')
            >> char!(' ')
            >> password:
                map_res!(
                    take_while1!(|c: u8| c.is_ascii_alphanumeric()),
                    std::str::from_utf8
                )
            >> opt!(complete!(call!(line_ending)))
            >> ((policy, password))
    ))
);

named!(
    parse_lines<Vec<(Policy, &str)>>,
    complete!(many0!(parse_line))
);

fn part_1(lines: &Vec<(Policy, &str)>) -> usize {
    lines
        .iter()
        .filter(|(policy, password)| {
            let (lower, upper) = policy.bounds;
            (lower..=upper).contains(&password.chars().filter(|c| c == &policy.character).count())
        })
        .count()
}

fn part_2(lines: &Vec<(Policy, &str)>) -> usize {
    lines
        .iter()
        .filter(|(policy, password)| {
            password
                .chars()
                .nth(policy.bounds.0 - 1)
                .filter(|c| c == &policy.character)
                .xor(
                    password
                        .chars()
                        .nth(policy.bounds.1 - 1)
                        .filter(|c| c == &policy.character),
                )
                .is_some()
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let (remainder, lines) = parse_lines(INPUT).context("failed to parse input")?;
    if !remainder.is_empty() {
        bail!(
            r#"input was not fully consumed (lines: {}, remaining: {:x?})"#,
            lines.len(),
            remainder
        );
    }

    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(&lines));

    Ok(())
}

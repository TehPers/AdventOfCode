use anyhow::Context;
use nom::{
    alt, call, char,
    character::streaming::{alpha1, digit1},
    do_parse, exact, map_res, named, recognize, separated_list1, tag, tuple, value,
};
use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

named!(parse_bag<&str, &str>,
    do_parse!(
        color: recognize!(tuple!(
            call!(alpha1), char!(' '), call!(alpha1)
        ))
        >> char!(' ')
        >> alt!(tag!("bags") | tag!("bag"))
        >> (color)
    )
);

named!(parse_line<&str, (&str, Vec<(&str, usize)>)>,
    exact!(do_parse!(
        bag: parse_bag
        >> tag!(" contain ")
        >> constraints: alt!(
            value!(Vec::new(), tag!("no other bags"))
            | separated_list1!(
                tag!(", "),
                do_parse!(
                    quantity: map_res!(
                        call!(digit1),
                        |s: &str| s.parse::<usize>()
                    )
                    >> char!(' ')
                    >> bag: parse_bag >>
                    ((bag, quantity))
                )
            )
        )
        >> char!('.')
        >> ((bag, constraints))
    ))
);

fn part1(input: &'static str) -> anyhow::Result<usize> {
    fn can_contain(
        constraints: &HashMap<&str, Vec<(&str, usize)>>,
        bag: &str,
        container: &str,
    ) -> bool {
        container == bag
            || constraints
                .get(container)
                .iter()
                .flat_map(|constraint| constraint.iter().map(|(container, _)| container))
                .any(|container| can_contain(constraints, bag, container))
    }

    let constraints = input
        .lines()
        .map(|s| {
            parse_line(s)
                .context("failure parsing input")
                .map(|(_, x)| x)
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let valid_bags = constraints
        .keys()
        .filter(|&bag| bag != &"shiny gold")
        .filter(|bag| can_contain(&constraints, "shiny gold", bag))
        .count();

    Ok(valid_bags)
}

fn part2(input: &'static str) -> anyhow::Result<usize> {
    fn needed_bags(constraints: &HashMap<&str, Vec<(&str, usize)>>, bag: &str) -> usize {
        constraints
            .get(bag)
            .iter()
            .flat_map(|constraint| constraint.iter())
            .map(|(child, quantity)| quantity * needed_bags(constraints, child))
            .sum::<usize>()
            + 1
    }

    let constraints = input
        .lines()
        .map(|s| {
            parse_line(s)
                .context("failure parsing input")
                .map(|(_, x)| x)
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(needed_bags(&constraints, "shiny gold") - 1)
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

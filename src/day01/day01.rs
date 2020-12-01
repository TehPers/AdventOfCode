use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(values: &[u32]) -> Option<(u32, u32)> {
    // Find the pair of numbers that add to 2020
    values
        .iter()
        .enumerate()
        .take(values.len() - 1)
        .find_map(|(i, a)| {
            values[i + 1..]
                .iter()
                .find(move |&b| a + b == 2020)
                .map(|b| (*a, *b))
        })
}

fn part2(values: &[u32]) -> Option<(u32, u32, u32)> {
    // Find the triple of numbers that add to 2020
    values
        .iter()
        .enumerate()
        .take(values.len() - 2)
        .find_map(|(i, a)| {
            values[i + 1..]
                .iter()
                .enumerate()
                .take(values.len() - i - 1)
                .find_map(|(j, b)| {
                    values[i + j + 1..]
                        .iter()
                        .find(move |&c| a + b + c == 2020)
                        .map(|c| (*a, *b, *c))
                })
        })
}

fn main() -> anyhow::Result<()> {
    // Read and parse the input file
    let input = BufReader::new(File::open("input.txt").context("could not open input file")?);
    let values = input
        .lines()
        .map(|line| {
            line.context("failure reading input file")
                .and_then(|line| line.parse().context("failure parsing input file"))
        })
        .collect::<Result<Vec<u32>, _>>()?;

    // Part 1
    let (a, b) = part1(&values).ok_or(anyhow!("no pair of numbers add up to 2020"))?;
    println!("part 1: {} * {} = {}", a, b, a * b);

    // Part 2
    let (a, b, c) = part2(&values).ok_or(anyhow!("no triple of numbers add up to 2020"))?;
    println!("part 2: {} * {} * {} = {}", a, b, c, a * b * c);

    Ok(())
}

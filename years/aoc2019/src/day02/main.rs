#[path = "../lib/intcode.rs"]
mod intcode;

use anyhow::Context;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn part1() -> anyhow::Result<usize> {
    let mut memory: Vec<usize> = INPUT.split(',').flat_map(|s| s.parse()).collect();
    memory[1] = 12;
    memory[2] = 2;
    intcode::run(&mut memory)?;
    Ok(memory[0])
}

fn part2() -> anyhow::Result<usize> {
    let source: Vec<usize> = INPUT.split(',').flat_map(|s| s.parse()).collect();
    let mut memory = vec![0; source.len()];
    let result = (0..100)
        .cartesian_product(0..100)
        .find(|&(noun, verb)| {
            memory.copy_from_slice(&source);
            memory[1] = noun;
            memory[2] = verb;
            intcode::run(&mut memory).unwrap();
            memory[0] == 19690720
        })
        .map(|(noun, verb)| 100 * noun + verb)
        .context("no valid pairs")?;

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1()?);
    println!("part 2: {}", part2()?);

    Ok(())
}

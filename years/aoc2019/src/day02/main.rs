#[path = "../lib/intcode.rs"]
mod intcode;

use anyhow::Context;
use intcode::MemoryValue;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn part1(input: &'static str) -> anyhow::Result<MemoryValue> {
    let mut memory: Vec<MemoryValue> = input.split(',').flat_map(|s| s.parse()).collect();
    memory[1] = 12;
    memory[2] = 2;
    intcode::run(
        &mut memory,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout().lock(),
    )?;
    Ok(memory[0])
}

fn part2(input: &'static str) -> anyhow::Result<MemoryValue> {
    let source: Vec<MemoryValue> = input.split(',').flat_map(|s| s.parse()).collect();
    let mut memory = vec![0; source.len()];
    let result = (0..100)
        .cartesian_product(0..100)
        .find(|&(noun, verb)| {
            memory.copy_from_slice(&source);
            memory[1] = noun;
            memory[2] = verb;
            intcode::run(
                &mut memory,
                &mut std::io::stdin().lock(),
                &mut std::io::stdout().lock(),
            )
            .unwrap();
            memory[0] == 19690720
        })
        .map(|(noun, verb)| 100 * noun + verb)
        .context("no valid pairs")?;

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

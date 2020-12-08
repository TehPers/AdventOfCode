#[path = "../lib/intcode.rs"]
mod intcode;

use anyhow::Context;
use intcode::{IntCodeComputer, MemoryValue};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn part1(memory: &mut [MemoryValue]) -> anyhow::Result<MemoryValue> {
    memory[1] = 12;
    memory[2] = 2;
    let mut computer = IntCodeComputer::new(memory);
    computer.run(&mut std::io::stdin().lock(), &mut std::io::stdout().lock())?;
    Ok(memory[0])
}

fn part2(source: &mut [MemoryValue]) -> anyhow::Result<MemoryValue> {
    let mut memory = vec![0; source.len()];
    let result = (0..100)
        .cartesian_product(0..100)
        .find(|&(noun, verb)| {
            memory.copy_from_slice(&source);
            memory[1] = noun;
            memory[2] = verb;
            let mut computer = IntCodeComputer::new(&mut memory);
            computer
                .run(&mut std::io::stdin().lock(), &mut std::io::stdout().lock())
                .is_ok()
                && memory[0] == 19690720
        })
        .map(|(noun, verb)| 100 * noun + verb)
        .context("no valid pairs")?;

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let mut memory: Vec<MemoryValue> = INPUT.split(',').flat_map(|s| s.parse()).collect();
    println!("part 1: {}", part1(&mut memory.clone())?);
    println!("part 2: {}", part2(&mut memory)?);

    Ok(())
}

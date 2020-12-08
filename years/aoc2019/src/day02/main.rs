#[path = "../lib/intcode.rs"]
mod intcode;

use anyhow::Context;
use intcode::{IntCodeComputer, MemoryValue};
use itertools::Itertools;
use std::io::BufRead;

const INPUT: &str = include_str!("input.txt");

fn run(memory: &mut [MemoryValue]) -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut istream = stdin.lock();
    let mut buffer = String::with_capacity(64);
    let mut computer = IntCodeComputer::new(
        memory,
        || match istream.read_line(&mut buffer)? {
            0 => Ok(None),
            _ => Ok(Some(buffer.trim().parse()?)),
        },
        |value| {
            println!("{}", value);
            Ok(())
        },
    );
    computer.run()?;
    Ok(())
}

fn part1(memory: &mut [MemoryValue]) -> anyhow::Result<MemoryValue> {
    memory[1] = 12;
    memory[2] = 2;
    run(memory)?;
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
            run(&mut memory).is_ok() && memory[0] == 19690720
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

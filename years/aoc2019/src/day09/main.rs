#[path = "../lib/intcode.rs"]
mod intcode;

use intcode::{IntCodeComputer, MemoryValue};
use std::io::BufReader;

const INPUT: &str = include_str!("input.txt");

fn run(memory: &mut [MemoryValue], input: &[u8]) -> anyhow::Result<()> {
    let mut computer = IntCodeComputer::new(memory);
    computer.run(&mut BufReader::new(input), &mut std::io::stdout().lock())?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut memory: Vec<MemoryValue> = INPUT.trim().split(',').flat_map(|s| s.parse()).collect();

    println!("part 1:");
    run(&mut memory.clone(), b"1")?;

    println!("part 2:");
    run(&mut memory, b"2")?;

    Ok(())
}

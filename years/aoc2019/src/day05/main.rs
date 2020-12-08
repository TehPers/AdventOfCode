#[path = "../lib/intcode.rs"]
mod intcode;

use intcode::MemoryValue;
use std::io::BufReader;

const INPUT: &str = include_str!("input.txt");

fn run(program: &'static str, input: &[u8]) -> anyhow::Result<()> {
    let mut memory: Vec<MemoryValue> = program.trim().split(',').flat_map(|s| s.parse()).collect();
    intcode::run(
        &mut memory,
        &mut BufReader::new(input),
        &mut std::io::stdout().lock(),
    )?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    println!("part 1:");
    run(INPUT, b"1")?;

    println!("part 2:");
    run(INPUT, b"5")?;

    Ok(())
}

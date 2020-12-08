#[path = "../lib/intcode.rs"]
mod intcode;

use intcode::{IntCodeComputer, MemoryValue};

const INPUT: &str = include_str!("input.txt");

fn run(memory: &mut [MemoryValue], input: &[MemoryValue]) -> anyhow::Result<()> {
    let mut input_index = 0;
    let mut computer = IntCodeComputer::new(
        memory,
        move || {
            input_index += 1;
            Ok(input.get(input_index - 1).copied())
        },
        |value| {
            println!("> {}", value);
            Ok(())
        },
    );
    computer.run()?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut memory: Vec<MemoryValue> = INPUT.trim().split(',').flat_map(|s| s.parse()).collect();

    println!("part 1:");
    run(&mut memory.clone(), &[1])?;

    println!("part 2:");
    run(&mut memory, &[2])?;

    Ok(())
}

use anyhow::{bail, Context};
use std::{collections::HashSet, convert::TryFrom};

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Instruction {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum RunResult {
    Loop(i32),
    Complete(i32),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(' ');
            let opcode = parts.next().context("missing opcode")?;
            let arg: i32 = parts.next().context("missing arg")?.parse()?;
            let instruction = match opcode {
                "nop" => Instruction::Nop(arg),
                "jmp" => Instruction::Jmp(arg),
                "acc" => Instruction::Acc(arg),
                _ => bail!("unknown opcode: {}", opcode),
            };

            Ok(instruction)
        })
        .collect::<Result<_, _>>()
}

fn run(instructions: &[Instruction]) -> anyhow::Result<RunResult> {
    let mut visited = HashSet::with_capacity(instructions.len());
    let mut ip = 0;
    let mut acc = 0;
    let result = loop {
        // check for loop
        if !visited.insert(ip) {
            break RunResult::Loop(acc);
        }

        match instructions.get(ip) {
            Some(Instruction::Nop(..)) => ip += 1,
            Some(Instruction::Acc(arg)) => {
                acc += arg;
                ip += 1;
            }
            Some(Instruction::Jmp(arg)) if *arg < 0 => ip -= usize::try_from(-arg)?,
            Some(Instruction::Jmp(arg)) => ip += usize::try_from(*arg)?,
            None if ip == instructions.len() => break RunResult::Complete(acc),
            None => bail!("unexpected EOF"),
        }
    };

    Ok(result)
}

fn part1(instructions: &[Instruction]) -> anyhow::Result<i32> {
    match run(instructions)? {
        RunResult::Loop(acc) => Ok(acc),
        RunResult::Complete(..) => bail!("expected infinite loop"),
    }
}

fn part2(instructions: &[Instruction]) -> anyhow::Result<i32> {
    let mut modified_instructions = vec![Instruction::Nop(0); instructions.len()];
    let result = instructions
        .iter()
        .enumerate()
        .find_map(|(addr, instruction)| {
            // prepare modified instructions
            match instruction {
                Instruction::Jmp(arg) => {
                    modified_instructions.copy_from_slice(instructions);
                    modified_instructions[addr] = Instruction::Nop(*arg);
                }
                Instruction::Nop(arg) => {
                    modified_instructions.copy_from_slice(instructions);
                    modified_instructions[addr] = Instruction::Jmp(*arg);
                }
                _ => return None,
            };

            // run modified instructions
            match run(&modified_instructions).ok()? {
                RunResult::Complete(acc) => Some(acc),
                _ => None,
            }
        })
        .context("no valid changes")?;

    Ok(result)
}

fn main() -> anyhow::Result<()> {
    let instructions = parse_input(INPUT)?;
    println!("part 1: {}", part1(&instructions)?);
    println!("part 2: {}", part2(&instructions)?);

    Ok(())
}

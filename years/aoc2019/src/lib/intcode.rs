use anyhow::{bail, Context};
use itertools::Itertools;
use std::{
    convert::TryInto,
    io::{BufRead, Write},
};

pub type MemoryValue = i32;

fn indirect(memory: &[MemoryValue], addr: usize) -> anyhow::Result<MemoryValue> {
    let addr: usize = memory
        .get(addr)
        .copied()
        .context("unexpected EOF")?
        .try_into()?;

    Ok(memory.get(addr).copied().context("unexpected EOF")?)
}

fn indirect_mut(memory: &mut [MemoryValue], addr: usize) -> anyhow::Result<&mut MemoryValue> {
    let addr: usize = memory
        .get(addr)
        .copied()
        .context("unexpected EOF")?
        .try_into()?;

    Ok(memory.get_mut(addr).context("unexpected EOF")?)
}

fn immediate(memory: &[MemoryValue], addr: usize) -> anyhow::Result<MemoryValue> {
    Ok(memory.get(addr).copied().context("unexpected EOF")?)
}

fn immediate_mut(memory: &mut [MemoryValue], addr: usize) -> anyhow::Result<&mut MemoryValue> {
    Ok(memory.get_mut(addr).context("unexpected EOF")?)
}

fn position_mode(
    value: MemoryValue,
) -> anyhow::Result<fn(&[MemoryValue], usize) -> anyhow::Result<MemoryValue>> {
    match value {
        0 => Ok(indirect),
        1 => Ok(immediate),
        _ => bail!("unknown position mode: {}", value),
    }
}

fn position_mode_mut(
    value: MemoryValue,
) -> anyhow::Result<fn(&mut [MemoryValue], usize) -> anyhow::Result<&mut MemoryValue>> {
    match value {
        0 => Ok(indirect_mut),
        1 => Ok(immediate_mut),
        _ => bail!("unknown position mode: {}", value),
    }
}

#[allow(dead_code)]
pub fn dump(memory: &[MemoryValue], ip: usize) {
    println!(
        "memory (len={}) = [\n{}\n]",
        memory.len(),
        memory
            .iter()
            .enumerate()
            .map(|(i, val)| if i == ip {
                format!("*{:5}", val)
            } else {
                format!(" {:5}", val)
            })
            .chunks(10)
            .into_iter()
            .map(|inner| inner.format(", "))
            .format(",\n")
    );
}

pub fn run(
    memory: &mut [MemoryValue],
    istream: &mut impl BufRead,
    ostream: &mut impl Write,
) -> anyhow::Result<()> {
    let mut ip = 0;
    let mut buffer = String::with_capacity(64);
    loop {
        let instruction = immediate(memory, ip)?;
        let opcode = instruction % 100;
        let (p1_mode, p2_mode, p3_mode) = (
            (instruction / 100) % 10,
            (instruction / 1000) % 10,
            (instruction / 10000) % 10,
        );
        let (p1_addr, p2_addr, p3_addr) = (ip + 1, ip + 2, ip + 3);
        match opcode {
            1 => {
                let a = position_mode(p1_mode)?(memory, p1_addr)?;
                let b = position_mode(p2_mode)?(memory, p2_addr)?;
                let target = position_mode_mut(p3_mode)?(memory, p3_addr)?;
                *target = a + b;
                ip += 4;
            }
            2 => {
                let a = position_mode(p1_mode)?(memory, p1_addr)?;
                let b = position_mode(p2_mode)?(memory, p2_addr)?;
                let target = position_mode_mut(p3_mode)?(memory, p3_addr)?;
                *target = a * b;
                ip += 4;
            }
            3 => {
                let target = position_mode_mut(p1_mode)?(memory, p1_addr)?;
                write!(ostream, "? ")?;
                ostream.flush()?;
                istream.read_line(&mut buffer)?;
                *target = buffer.trim_end().parse()?;
                ip += 2;
            }
            4 => {
                let value = position_mode_mut(p1_mode)?(memory, p1_addr)?;
                writeln!(ostream, "> {}", value)?;
                ip += 2;
            }
            5 => {
                let condition = position_mode(p1_mode)?(memory, p1_addr)?;
                let target: usize = position_mode(p2_mode)?(memory, p2_addr)?.try_into()?;
                if condition != 0 {
                    ip = target;
                } else {
                    ip += 3;
                }
            }
            6 => {
                let condition = position_mode(p1_mode)?(memory, p1_addr)?;
                let target: usize = position_mode(p2_mode)?(memory, p2_addr)?.try_into()?;
                if condition == 0 {
                    ip = target;
                } else {
                    ip += 3;
                }
            }
            7 => {
                let a = position_mode(p1_mode)?(memory, p1_addr)?;
                let b = position_mode(p2_mode)?(memory, p2_addr)?;
                let target = position_mode_mut(p3_mode)?(memory, p3_addr)?;
                *target = if a < b { 1 } else { 0 };
                ip += 4;
            }
            8 => {
                let a = position_mode(p1_mode)?(memory, p1_addr)?;
                let b = position_mode(p2_mode)?(memory, p2_addr)?;
                let target = position_mode_mut(p3_mode)?(memory, p3_addr)?;
                *target = if a == b { 1 } else { 0 };
                ip += 4;
            }
            99 => break,
            _ => bail!("unknown opcode: {}", opcode),
        }
    }

    Ok(())
}

use anyhow::{bail, Context};

const INPUT: &str = include_str!("input.txt");

fn run(memory: &mut [usize]) -> anyhow::Result<()> {
    fn indirect(memory: &[usize], addr: usize) -> anyhow::Result<usize> {
        Ok(memory
            .get(addr)
            .and_then(|&pos| memory.get(pos))
            .copied()
            .context("unexpected EOF")?)
    }

    fn indirect_mut(memory: &mut [usize], addr: usize) -> anyhow::Result<&mut usize> {
        Ok(memory
            .get(addr)
            .cloned()
            .and_then(move |pos| memory.get_mut(pos))
            .context("unexpected EOF")?)
    }

    let mut ip = 0;
    loop {
        match memory.get(ip) {
            Some(1) => {
                let a = indirect(memory, ip + 1)?;
                let b = indirect(memory, ip + 2)?;
                let target = indirect_mut(memory, ip + 3)?;
                *target = a + b;
                ip += 4;
            }
            Some(2) => {
                let a = indirect(memory, ip + 1)?;
                let b = indirect(memory, ip + 2)?;
                let target = indirect_mut(memory, ip + 3)?;
                *target = a * b;
                ip += 4;
            }
            Some(99) => break,
            Some(n @ _) => bail!("unknown opcode: {}", n),
            None => bail!("unexpected EOF"),
        }
    }

    Ok(())
}

fn part1() -> anyhow::Result<usize> {
    let mut memory: Vec<usize> = INPUT.split(',').flat_map(|s| s.parse()).collect();
    memory[1] = 12;
    memory[2] = 2;
    run(&mut memory)?;
    Ok(memory[0])
}

fn part2() -> anyhow::Result<usize> {
    let source: Vec<usize> = INPUT.split(',').flat_map(|s| s.parse()).collect();
    let mut memory = vec![0; source.len()];
    let result = (0..100)
        .flat_map(|noun| (0..100).map(move |verb| (noun, verb)))
        .find(|&(noun, verb)| {
            memory.copy_from_slice(&source);
            memory[1] = noun;
            memory[2] = verb;
            run(&mut memory).unwrap();
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

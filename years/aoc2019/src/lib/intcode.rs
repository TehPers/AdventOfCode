use anyhow::{Context, bail};

pub fn run(memory: &mut [usize]) -> anyhow::Result<()> {
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
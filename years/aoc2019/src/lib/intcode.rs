use anyhow::bail;
use itertools::Itertools;
use std::{
    collections::HashMap,
    convert::TryInto,
    io::{BufRead, Write},
};

pub type MemoryValue = i64;

#[derive(Debug)]
pub struct IntCodeComputer<'m> {
    pub base_memory: &'m mut [MemoryValue],
    pub additional_memory: HashMap<usize, MemoryValue>,
    pub relative_base: i64,
    pub ip: usize,
    input_buffer: String,
}

impl<'m> IntCodeComputer<'m> {
    pub fn new(base_memory: &'m mut [MemoryValue]) -> Self {
        IntCodeComputer {
            base_memory,
            additional_memory: HashMap::new(),
            relative_base: 0,
            ip: 0,
            input_buffer: String::with_capacity(64),
        }
    }

    fn immediate(&mut self, addr: usize) -> MemoryValue {
        match self.base_memory.get(addr) {
            Some(&value) => value,
            None => self.additional_memory.get(&addr).copied().unwrap_or(0),
        }
    }

    fn immediate_mut(&mut self, addr: usize) -> &mut MemoryValue {
        match self.base_memory.get_mut(addr) {
            Some(value) => value,
            None => self.additional_memory.entry(addr).or_insert(0),
        }
    }

    fn indirect(&mut self, addr: usize) -> anyhow::Result<MemoryValue> {
        let addr = self.immediate(addr).try_into()?;
        Ok(self.immediate(addr))
    }

    fn indirect_mut(&mut self, addr: usize) -> anyhow::Result<&mut MemoryValue> {
        let addr = self.immediate(addr).try_into()?;
        Ok(self.immediate_mut(addr))
    }

    fn relative(&mut self, addr: usize) -> anyhow::Result<MemoryValue> {
        let offset = self.immediate(addr);
        let addr = (self.relative_base + offset).try_into()?;
        Ok(self.immediate(addr))
    }

    fn relative_mut(&mut self, addr: usize) -> anyhow::Result<&mut MemoryValue> {
        let offset = self.immediate(addr);
        let addr = (self.relative_base + offset).try_into()?;
        Ok(self.immediate_mut(addr))
    }

    fn get_parameter(&mut self, mode: MemoryValue, addr: usize) -> anyhow::Result<MemoryValue> {
        match mode {
            0 => self.indirect(addr),
            1 => Ok(self.immediate(addr)),
            2 => self.relative(addr),
            _ => bail!("unknown position mode: {}", addr),
        }
    }

    fn get_parameter_mut(
        &mut self,
        mode: MemoryValue,
        addr: usize,
    ) -> anyhow::Result<&mut MemoryValue> {
        match mode {
            0 => self.indirect_mut(addr),
            1 => Ok(self.immediate_mut(addr)),
            2 => self.relative_mut(addr),
            _ => bail!("unknown position mode: {}", addr),
        }
    }

    pub fn step(
        &mut self,
        istream: &mut impl BufRead,
        ostream: &mut impl Write,
    ) -> anyhow::Result<bool> {
        let instruction = self.immediate(self.ip);
        let opcode = instruction % 100;
        let (p1_mode, p2_mode, p3_mode) = (
            (instruction / 100) % 10,
            (instruction / 1000) % 10,
            (instruction / 10000) % 10,
        );
        let (p1_addr, p2_addr, p3_addr) = (self.ip + 1, self.ip + 2, self.ip + 3);
        match opcode {
            1 => {
                // add
                let a = self.get_parameter(p1_mode, p1_addr)?;
                let b = self.get_parameter(p2_mode, p2_addr)?;
                let target = self.get_parameter_mut(p3_mode, p3_addr)?;
                *target = a + b;

                self.ip += 4;
            }
            2 => {
                // multiply
                let a = self.get_parameter(p1_mode, p1_addr)?;
                let b = self.get_parameter(p2_mode, p2_addr)?;
                let target = self.get_parameter_mut(p3_mode, p3_addr)?;
                *target = a * b;

                self.ip += 4;
            }
            3 => {
                // read int
                write!(ostream, "? ")?;
                ostream.flush()?;
                istream.read_line(&mut self.input_buffer)?;

                let input = self.input_buffer.trim_end().parse()?;
                let target = self.get_parameter_mut(p1_mode, p1_addr)?;
                *target = input;

                self.ip += 2;
            }
            4 => {
                // write int
                let value = self.get_parameter_mut(p1_mode, p1_addr)?;
                writeln!(ostream, "> {}", value)?;

                self.ip += 2;
            }
            5 => {
                // jump if true
                let condition = self.get_parameter(p1_mode, p1_addr)?;
                let target: usize = self.get_parameter(p2_mode, p2_addr)?.try_into()?;

                if condition != 0 {
                    self.ip = target;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                // jump if false
                let condition = self.get_parameter(p1_mode, p1_addr)?;
                let target: usize = self.get_parameter(p2_mode, p2_addr)?.try_into()?;

                if condition == 0 {
                    self.ip = target;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                // less than
                let a = self.get_parameter(p1_mode, p1_addr)?;
                let b = self.get_parameter(p2_mode, p2_addr)?;
                let target = self.get_parameter_mut(p3_mode, p3_addr)?;
                *target = if a < b { 1 } else { 0 };

                self.ip += 4;
            }
            8 => {
                // equals
                let a = self.get_parameter(p1_mode, p1_addr)?;
                let b = self.get_parameter(p2_mode, p2_addr)?;
                let target = self.get_parameter_mut(p3_mode, p3_addr)?;
                *target = if a == b { 1 } else { 0 };

                self.ip += 4;
            }
            9 => {
                // adjust relative base
                let offset = self.get_parameter(p1_mode, p1_addr)?;
                self.relative_base += offset;

                self.ip += 2;
            }
            99 => {
                return Ok(false);
            }
            _ => bail!("unknown opcode: {}", opcode),
        }

        Ok(true)
    }

    pub fn run(
        &mut self,
        istream: &mut impl BufRead,
        ostream: &mut impl Write,
    ) -> anyhow::Result<()> {
        while self.step(istream, ostream)? {}
        Ok(())
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        println!("ip = {}", self.ip);
        println!("relative base = {}", self.relative_base);
        println!(
            "base memory = [\n{}\n]",
            self.base_memory
                .iter()
                .enumerate()
                .map(|(i, value)| if i == self.ip {
                    format!("*[{:4}]: {:5}", i, value)
                } else {
                    format!(" [{:4}]: {:5}", i, value)
                })
                .chunks(5)
                .into_iter()
                .map(|inner| inner.format(", "))
                .format(",\n")
        );
        println!(
            "additional memory = {{\n{}\n}}",
            self.additional_memory
                .iter()
                .sorted()
                .map(|(&k, &v)| if k == self.ip {
                    format!("*[{:4}]: {:5}", k, v)
                } else {
                    format!(" [{:4}]: {:5}", k, v)
                })
                .chunks(5)
                .into_iter()
                .map(|inner| inner.format(", "))
                .format(",\n")
        );
    }
}

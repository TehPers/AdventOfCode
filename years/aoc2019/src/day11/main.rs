#[path = "../lib/intcode.rs"]
mod intcode;

use anyhow::bail;
use intcode::{IntCodeComputer, MemoryValue};
use itertools::{Itertools, MinMaxResult};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate_cw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn rotate_ccw(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Color {
    Black,
    White,
}

impl From<Color> for MemoryValue {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct State {
    position: (i32, i32),
    direction: Direction,
    painted: HashMap<(i32, i32), Color>,
}

impl Default for State {
    fn default() -> Self {
        State {
            position: (0, 0),
            direction: Direction::Up,
            painted: HashMap::new(),
        }
    }
}

fn run(memory: &mut [MemoryValue], state: &mut State) -> anyhow::Result<()> {
    let state = Rc::new(RefCell::new(state));
    let mut computer = IntCodeComputer::new(
        memory,
        {
            let state = state.clone();
            move || {
                let state = state.borrow();
                Ok(state
                    .painted
                    .get(&state.position)
                    .copied()
                    .map(MemoryValue::from)
                    .or(Some(0)))
            }
        },
        {
            let mut color = None;
            move |value| match color.take() {
                None => {
                    color = Some(value);
                    Ok(())
                }
                Some(color) => {
                    let mut state = state.borrow_mut();
                    let position = state.position;
                    let color = match color {
                        0 => Color::Black,
                        1 => Color::White,
                        _ => bail!("unknown color: {}", color),
                    };
                    state.painted.insert(position, color);

                    match value {
                        0 => state.direction = state.direction.rotate_ccw(),
                        1 => state.direction = state.direction.rotate_cw(),
                        _ => bail!("unknown direction: {}", value),
                    }
                    state.position = match state.direction {
                        Direction::Up => (state.position.0, state.position.1 - 1),
                        Direction::Right => (state.position.0 + 1, state.position.1),
                        Direction::Down => (state.position.0, state.position.1 + 1),
                        Direction::Left => (state.position.0 - 1, state.position.1),
                    };

                    Ok(())
                }
            }
        },
    );
    computer.run()?;

    Ok(())
}

fn part1(memory: &mut [MemoryValue]) -> anyhow::Result<usize> {
    let mut state = State::default();
    run(memory, &mut state)?;
    Ok(state.painted.len())
}

fn part2(memory: &mut [MemoryValue]) -> anyhow::Result<()> {
    let mut state = State::default();
    state.painted.insert((0, 0), Color::White);
    run(memory, &mut state)?;

    let xs = match state.painted.keys().map(|&(x, _)| x).minmax() {
        MinMaxResult::NoElements => bail!("no painted tiles"),
        MinMaxResult::OneElement(value) => value..=value,
        MinMaxResult::MinMax(min, max) => min..=max,
    };

    let ys = match state.painted.keys().map(|&(_, y)| y).minmax() {
        MinMaxResult::NoElements => bail!("no painted tiles"),
        MinMaxResult::OneElement(value) => value..=value,
        MinMaxResult::MinMax(min, max) => min..=max,
    };

    for y in ys {
        for x in xs.clone() {
            let color = state.painted.get(&(x, y)).copied().unwrap_or(Color::Black);
            print!(
                "{}",
                match color {
                    Color::Black => ".",
                    Color::White => "#",
                }
            );
        }

        println!();
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut memory: Vec<MemoryValue> = INPUT.trim().split(',').flat_map(|s| s.parse()).collect();
    println!("part 1: {}", part1(&mut memory.clone())?);
    println!("part 2:");
    part2(&mut memory)?;

    Ok(())
}

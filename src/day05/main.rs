use anyhow::Context;
use std::{
    io::{BufRead, BufReader},
    ops::Range,
};

const INPUT: &[u8] = include_bytes!("input.txt");

fn partition(seat: &str, range: Range<u32>) -> Option<u32> {
    match seat.as_bytes().first() {
        Some(b'F') | Some(b'L') => {
            partition(&seat[1..], range.start..((range.start + range.end + 1) / 2))
        }
        Some(b'B') | Some(b'R') => {
            partition(&seat[1..], ((range.start + range.end + 1) / 2)..range.end)
        }
        None => Some(range.start),
        _ => None,
    }
}

fn get_seat_id(seat: &str) -> Option<u32> {
    let row = partition(&seat[0..7], 0..128)?;
    let col = partition(&seat[7..], 0..8)?;
    Some(row * 8 + col)
}

fn part1(input: &Vec<String>) -> anyhow::Result<u32> {
    let max_id = input
        .iter()
        .filter(|s| !s.is_empty())
        .flat_map(|s| get_seat_id(s))
        .max()
        .context("no valid seats")?;

    Ok(max_id)
}

fn part2(input: &Vec<String>) -> anyhow::Result<u32> {
    let mut seat_ids: Vec<_> = input
        .iter()
        .filter(|s| !s.is_empty())
        .flat_map(|s| get_seat_id(s))
        .collect();

    seat_ids.sort_unstable();
    let first = *seat_ids.first().context("no valid seats")?;
    let seat_before_gap =
        seat_ids
            .into_iter()
            .fold(first, |prev, cur| if prev + 1 == cur { cur } else { prev });
    Ok(seat_before_gap + 1)
}

fn main() -> anyhow::Result<()> {
    let input = BufReader::new(INPUT)
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .context("failure reading input file")?;

    println!("part 1: {}", part1(&input)?);
    println!("part 2: {}", part2(&input)?);

    Ok(())
}

use anyhow::Context;
use std::io::{BufRead, BufReader};

const INPUT: &[u8] = include_bytes!("input.txt");

fn count_trees(lines: &Vec<String>, right: usize, down: usize) -> usize {
    lines
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, line)| line.as_bytes()[(i * right) % line.len()])
        .filter(|&c| c == b'#')
        .count()
}

fn solve(lines: &Vec<String>, steps: Vec<(usize, usize)>) -> usize {
    steps
        .into_iter()
        .map(|(right, down)| count_trees(&lines, right, down))
        .product()
}

fn main() -> anyhow::Result<()> {
    let input = BufReader::new(INPUT);
    let lines = input
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .context("failure reading input file")?;

    println!("part 1: {}", solve(&lines, vec![(3, 1)]));
    println!(
        "part 2: {}",
        solve(&lines, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
    );

    Ok(())
}

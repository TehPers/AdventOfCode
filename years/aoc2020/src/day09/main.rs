use anyhow::{bail, Context};
use itertools::{Itertools, MinMaxResult};

const INPUT: &str = include_str!("input.txt");

fn part1(values: &[usize]) -> anyhow::Result<usize> {
    let result = values
        .iter()
        .enumerate()
        .skip(25)
        .map(|(i, &x)| (&values[i - 25..i], x))
        .find(|&(prev, x)| {
            !prev
                .iter()
                .cartesian_product(prev)
                .any(|(&a, &b)| a + b == x)
        })
        .context("no gaps found")?;

    Ok(result.1)
}

fn part2(values: &[usize], target: usize) -> anyhow::Result<usize> {
    let (i, j) = (0..values.len())
        .flat_map(|i| (i + 1..values.len()).map(move |j| (i, j)))
        .find(|&(i, j)| values[i..j].iter().sum::<usize>() == target)
        .context("no range found")?;

    match values[i..j].iter().minmax() {
        MinMaxResult::MinMax(low, high) => Ok(low + high),
        _ => bail!("not enough elements in range"),
    }
}

fn main() -> anyhow::Result<()> {
    let values: Vec<_> = INPUT.lines().map(str::parse).collect::<Result<_, _>>()?;
    let target = part1(&values)?;
    println!("part 1: {}", target);
    println!("part 2: {}", part2(&values, target)?);

    Ok(())
}

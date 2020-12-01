use anyhow::{anyhow, Context};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(values: &[u32]) -> Option<(u32, u32)> {
    if values.is_empty() {
        return None;
    }

    // Find the pair of numbers that add to 2020
    values.iter().enumerate().find_map(|(i, a)| {
        if i + 1 < values.len() {
            values[i + 1..]
                .iter()
                .find(move |&b| a + b == 2020)
                .map(|b| (*a, *b))
        } else {
            None
        }
    })
}

fn part2(values: &[u32]) -> Option<(u32, u32, u32)> {
    if values.is_empty() {
        return None;
    }

    // Find the triple of numbers that add to 2020
    values.iter().enumerate().find_map(|(i, a)| {
        if i + 2 < values.len() {
            values[i + 1..].iter().enumerate().find_map(|(j, b)| {
                if i + j + 2 < values.len() {
                    values[i + j + 2..]
                        .iter()
                        .find(move |&c| a + b + c == 2020)
                        .map(|c| (*a, *b, *c))
                } else {
                    None
                }
            })
        } else {
            None
        }
    })
}

fn main() -> anyhow::Result<()> {
    // Read and parse the input file
    let input = BufReader::new(File::open("input.txt").context("could not open input file")?);
    let values = input
        .lines()
        .map(|line| {
            line.context("failure reading input file")
                .and_then(|line| line.parse().context("failure parsing input file"))
        })
        .collect::<Result<Vec<u32>, _>>()?;

    // Part 1
    let (a, b) = part1(&values).ok_or(anyhow!("no pair of numbers add up to 2020"))?;
    println!("part 1: {} * {} = {}", a, b, a * b);

    // Part 2
    let (a, b, c) = part2(&values).ok_or(anyhow!("no triple of numbers add up to 2020"))?;
    println!("part 2: {} * {} * {} = {}", a, b, c, a * b * c);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_works() {
        assert_eq!(part1(&[1010, 1010]), Some((1010, 1010)));
        assert_eq!(part1(&[1010, 1011]), None);
        assert_eq!(
            part1(&[1, 2, 1, 0, 1, 1009, 1011, 4, 8, 9]),
            Some((1009, 1011))
        );
        assert_eq!(part1(&[1009, 1011, 4, 8, 9]), Some((1009, 1011)));
        assert_eq!(part1(&[1, 2, 1, 0, 1, 1009, 1011]), Some((1009, 1011)));
        assert_eq!(part1(&[1009, 4, 9, 12, 1011]), Some((1009, 1011)));
        assert_eq!(part1(&[1, 2, 1, 0, 1, 2020, 33, 4, 8, 9]), Some((0, 2020)));
        assert_eq!(part1(&[1, 2, 1, 9, 1, 2020, 33, 4, 8, 9]), None);
        assert_eq!(part1(&[]), None);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&[2000, 10, 10]), Some((2000, 10, 10)));
        assert_eq!(part2(&[2000, 10, 11]), None);
        assert_eq!(
            part2(&[1, 2, 1, 0, 2000, 11, 9, 4, 8, 9]),
            Some((2000, 11, 9))
        );
        assert_eq!(part2(&[2000, 11, 9, 4, 8, 9]), Some((2000, 11, 9)));
        assert_eq!(part2(&[1, 2, 1, 0, 2000, 11, 9]), Some((2000, 11, 9)));
        assert_eq!(part2(&[2000, 4, 11, 8, 6, 9]), Some((2000, 11, 9)));
        assert_eq!(part2(&[2020, 0, 0]), Some((2020, 0, 0)));
        assert_eq!(part2(&[2020, 0, 1]), None);
        assert_eq!(part2(&[2020, 4, 1]), None);
        assert_eq!(part2(&[]), None);
    }
}

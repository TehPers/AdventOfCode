use anyhow::{bail, Context};
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = include_str!("input.txt");

fn parse_orbits(input: &str) -> anyhow::Result<HashMap<&str, &str>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(')');
            let center = parts.next().context("missing center")?;
            let satellite = parts.next().context("missing satellite")?;
            Ok((satellite, center))
        })
        .collect()
}

fn part1(orbits: &HashMap<&str, &str>) -> usize {
    fn count_orbits(orbits: &HashMap<&str, &str>, object: &str) -> usize {
        match orbits.get(object) {
            Some(center) => 1 + count_orbits(orbits, center),
            None => 0,
        }
    }

    orbits
        .keys()
        .map(|object| count_orbits(orbits, object))
        .sum()
}

fn part2(orbits: &HashMap<&str, &str>) -> anyhow::Result<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::with_capacity(orbits.len());
    queue.push_back((0, "YOU"));
    while let Some((dist, object)) = queue.pop_front() {
        if object == "SAN" {
            return Ok(dist - 2);
        }

        if !visited.insert(object) {
            continue;
        }

        if let Some(center) = orbits.get(object) {
            queue.push_back((dist + 1, center));
        }

        for (satellite, _) in orbits.iter().filter(|(_, &v)| v == object) {
            queue.push_back((dist + 1, satellite));
        }
    }

    bail!("no path found");
}

fn main() -> anyhow::Result<()> {
    let orbits = parse_orbits(INPUT)?;
    println!("part 1: {}", part1(&orbits));
    println!("part 2: {}", part2(&orbits)?);

    Ok(())
}

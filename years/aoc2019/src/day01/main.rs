const INPUT: &str = include_str!("input.txt");

fn part1(input: &'static str) -> i32 {
    input
        .lines()
        .flat_map(|line| line.parse())
        .map(|n: i32| n / 3 - 2)
        .sum()
}

fn part2(input: &'static str) -> i32 {
    input
        .lines()
        .flat_map(|line| line.parse())
        .flat_map(|n: i32| {
            itertools::iterate(n, |&n| n / 3 - 2)
                .skip(1)
                .take_while(|&n| n > 0)
        })
        .sum()
}

pub fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part1(INPUT), 3249817);
        assert_eq!(part2(INPUT), 4871866);
    }
}

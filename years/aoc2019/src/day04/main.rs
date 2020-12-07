use itertools::Itertools;

fn digits(mut n: u32) -> impl Clone + Iterator<Item = u32> {
    std::iter::from_fn(move || {
        if n > 0 {
            let result = n % 10;
            n /= 10;
            Some(result)
        } else {
            None
        }
    })
}

fn part1(range: impl Iterator<Item = u32>) -> usize {
    range
        .map(digits)
        .filter(|digits| {
            // check if digits are monotonically decreasing
            digits
                .clone()
                .fold((true, None), |acc, cur| match acc {
                    (false, _) => (false, Some(cur)),
                    (true, None) => (true, Some(cur)),
                    (true, Some(prev)) => (cur <= prev, Some(cur)),
                })
                .0
        })
        .map(|digits| {
            // check if there is a pair in the digits
            digits
                .group_by(|&n| n)
                .into_iter()
                .any(|(_, group)| group.count() >= 2)
        })
        .filter(|&has_pair| has_pair)
        .count()
}

fn part2(range: impl Iterator<Item = u32>) -> usize {
    range
        .map(digits)
        .filter(|digits| {
            // check if digits are monotonically decreasing
            digits
                .clone()
                .fold((true, None), |acc, cur| match acc {
                    (false, _) => (false, Some(cur)),
                    (true, None) => (true, Some(cur)),
                    (true, Some(prev)) => (cur <= prev, Some(cur)),
                })
                .0
        })
        .map(|digits| {
            // check if there is an exact pair in the digits
            digits
                .group_by(|&n| n)
                .into_iter()
                .any(|(_, group)| group.count() == 2)
        })
        .filter(|&has_pair| has_pair)
        .count()
}

fn main() {
    let input = 357253..=892942;
    println!("part 1: {}", part1(input.clone()));
    println!("part 2: {}", part2(input));
}

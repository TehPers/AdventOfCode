use anyhow::{anyhow, bail, Context};
use nom::{
    call,
    character::complete::{anychar, char, digit1},
    combinator::{complete, map},
    do_parse, map_opt, map_res,
    multi::separated_list0,
    named, IResult,
};

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Instruction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Segment {
    Horizontal { x: (i32, i32), y: i32 },
    Vertical { x: i32, y: (i32, i32) },
}

impl Segment {
    pub fn start(self) -> (i32, i32) {
        match self {
            Segment::Horizontal { x: (x, _), y } => (x, y),
            Segment::Vertical { x, y: (y, _) } => (x, y),
        }
    }

    pub fn len(self) -> i32 {
        match self {
            Segment::Horizontal { x: (x1, x2), .. } => (x1 - x2).abs(),
            Segment::Vertical { y: (y1, y2), .. } => (y1 - y2).abs(),
        }
    }

    pub fn point_intersection(self, other: Segment) -> Option<(i32, i32)> {
        match (self, other) {
            (Segment::Horizontal { x: x1, y: y1 }, Segment::Vertical { x: x2, y: y2 })
                if x2 >= x1.0.min(x1.1)
                    && x2 <= x1.0.max(x1.1)
                    && y1 >= y2.0.min(y2.1)
                    && y1 <= y2.0.max(y2.1) =>
            {
                Some((x2, y1))
            }
            (Segment::Vertical { x: x1, y: y1 }, Segment::Horizontal { x: x2, y: y2 })
                if x1 >= x2.0.min(x2.1)
                    && x1 <= x2.0.max(x2.1)
                    && y2 >= y1.0.min(y1.1)
                    && y2 <= y1.0.max(y1.1) =>
            {
                Some((x1, y2))
            }
            _ => None,
        }
    }
}

named!(
    parse_instruction<&str, Instruction>,
    map_opt!(
        do_parse!(
            direction: call!(anychar)
                >> amount:
                    map_res!(call!(digit1), |s: &str| s.parse())
                >> (direction, amount)
        ),
        |(direction, amount)| match direction {
            'L' => Some(Instruction::Left(amount)),
            'R' => Some(Instruction::Right(amount)),
            'U' => Some(Instruction::Up(amount)),
            'D' => Some(Instruction::Down(amount)),
            _ => None,
        }
    )
);

fn parse_segments(input: &str) -> IResult<&str, Vec<Segment>> {
    let mut position = (0, 0);
    let (input, segments) = separated_list0(
        complete(char(',')),
        map(
            complete(parse_instruction),
            |instruction| match instruction {
                Instruction::Left(amount) => {
                    let segment = Segment::Horizontal {
                        x: (position.0, position.0 - amount),
                        y: position.1,
                    };
                    position = (position.0 - amount, position.1);
                    segment
                }
                Instruction::Right(amount) => {
                    let segment = Segment::Horizontal {
                        x: (position.0, position.0 + amount),
                        y: position.1,
                    };
                    position = (position.0 + amount, position.1);
                    segment
                }
                Instruction::Up(amount) => {
                    let segment = Segment::Vertical {
                        x: position.0,
                        y: (position.1, position.1 - amount),
                    };
                    position = (position.0, position.1 - amount);
                    segment
                }
                Instruction::Down(amount) => {
                    let segment = Segment::Vertical {
                        x: position.0,
                        y: (position.1, position.1 + amount),
                    };
                    position = (position.0, position.1 + amount);
                    segment
                }
            },
        ),
    )(input)?;

    Ok((input, segments))
}

fn manhattan_dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part1(input: &str) -> anyhow::Result<i32> {
    let paths = input
        .split('\n')
        .map(|line| {
            parse_segments(&line)
                .map(|(_, segment)| segment)
                .map_err(|error| anyhow!("failure parsing instructions: {}", error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if paths.len() != 2 {
        bail!("two paths required, found {}", paths.len());
    }

    let first_path = paths.get(0).context("not enough paths")?;
    let second_path = paths.get(1).context("not enough paths")?;
    first_path
        .iter()
        .flat_map(|s1| second_path.iter().map(move |s2| (s1, s2)))
        .filter_map(|(s1, s2)| s1.point_intersection(*s2))
        .map(|(x, y)| x.abs() + y.abs())
        .filter(|n| *n != 0)
        .fold(None, |smallest, cur| match (smallest, cur) {
            (None, cur) => Some(cur),
            (Some(a), cur) if cur < a => Some(cur),
            _ => smallest,
        })
        .context("not enough path segments")
}

fn part2(input: &str) -> anyhow::Result<i32> {
    let paths = input
        .split('\n')
        .map(|line| {
            parse_segments(&line)
                .map(|(_, segment)| segment)
                .map_err(|error| anyhow!("failure parsing instructions: {}", error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if paths.len() != 2 {
        bail!("two paths required, found {}", paths.len());
    }

    let first_path = paths.get(0).context("not enough paths")?;
    let second_path = paths.get(1).context("not enough paths")?;
    first_path
        .iter()
        .scan(0i32, |steps: &mut i32, segment: &Segment| {
            *steps += segment.len();
            Some((*steps, segment))
        })
        .flat_map(|s1| {
            second_path
                .iter()
                .scan(0i32, |steps: &mut i32, segment: &Segment| {
                    *steps += segment.len();
                    Some((*steps, segment))
                })
                .map(move |s2| (s1, s2))
        })
        .filter_map(|((steps1, &segment1), (steps2, &segment2))| {
            segment1
                .point_intersection(segment2)
                .filter(|&(x, y)| x != 0 || y != 0)
                .map(|intersection| {
                    steps1
                        + steps2
                        + manhattan_dist(segment1.start(), intersection)
                        + manhattan_dist(segment2.start(), intersection)
                        - segment1.len()
                        - segment2.len()
                })
        })
        .fold(None, |smallest, cur| match (smallest, cur) {
            (None, cur) => Some(cur),
            (Some(a), cur) if cur < a => Some(cur),
            _ => smallest,
        })
        .context("not enough path segments")
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1(INPUT)?);
    println!("part 2: {}", part2(INPUT)?);

    Ok(())
}

use std::{collections::HashSet, iter::Iterator};

const INPUT: &str = include_str!("input.txt");

struct Groups<'i> {
    input: &'i [u8],
}

impl<'i> From<&'i str> for Groups<'i> {
    fn from(s: &'i str) -> Self {
        Groups {
            input: s.as_bytes(),
        }
    }
}

impl<'i> Iterator for Groups<'i> {
    type Item = Answers<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        let mut newline = false;
        while let Some(&b) = self.input.get(i) {
            i += 1;

            match b {
                b'\n' if newline => break,
                b'\n' => {
                    newline = true;
                }
                _ => {
                    newline = false;
                }
            }
        }

        if i == 0 {
            None
        } else {
            let (result, remainder) = self.input.split_at(i);
            self.input = remainder;
            Some(Answers { input: result })
        }
    }
}

struct Answers<'i> {
    input: &'i [u8],
}

impl<'i> Iterator for Answers<'i> {
    type Item = &'i [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        while let Some(&b) = self.input.get(i) {
            if b == b'\n' {
                break;
            }

            i += 1;
        }

        if i == 0 {
            None
        } else {
            let result = &self.input[0..i];
            self.input = &self.input[i + 1..];
            Some(result)
        }
    }
}

fn part1(input: &str) -> usize {
    Groups::from(input)
        .map(|group| {
            group
                .fold(HashSet::new(), |mut set, cur| {
                    cur.iter().for_each(|c| {
                        set.insert(c);
                    });
                    set
                })
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    Groups::from(input)
        .map(|group| {
            group
                .fold((b'a'..=b'z').collect::<HashSet<_>>(), |set, cur| {
                    set.intersection(&cur.iter().copied().collect())
                        .copied()
                        .collect()
                })
                .len()
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

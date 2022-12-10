#![feature(slice_partition_dedup)]
#![feature(iter_array_chunks)]

use std::{collections::BTreeSet, iter::Peekable};

fn day1(input: &str) -> (usize, usize) {
    let result = input
        .split("\n\n")
        .map(|l| {
            l.lines()
                .map(|s| s.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .fold([0usize; 3], |mut state, nb| {
            if let Some(pos) = state.iter().position(|prev| *prev < nb) {
                for i in (pos..state.len() - 1).rev() {
                    state[i + 1] = state[i]
                }
                state[pos] = nb;
            }
            state
        });
    (result[0], result.iter().sum::<usize>())
}

fn day2(input: &[u8]) -> (usize, usize) {
    input.split(|b| *b == b'\n').fold((0, 0), |sum, l| {
        let opp = (l[0] - b'A') as usize;
        let this = (l[2] - b'X') as usize;
        let loose = [2, 0, 1][opp];
        let win = [1, 2, 0][opp];
        let strat = match this {
            0 => loose,
            1 => opp + 3,
            _ => win + 6,
        };
        let score = this
            + if this == win {
                6
            } else if this == opp {
                3
            } else {
                0
            };
        (sum.0 + score + 1, sum.1 + strat + 1)
    })
}

fn day3(input: &[u8]) -> (usize, usize) {
    fn bitset(s: &[u8]) -> u64 {
        let mut set = 0;
        for b in s {
            let offset = match b {
                b'a'..=b'z' => b - b'a' + 1,
                _ => b - b'A' + 27,
            };
            set |= 1 << offset
        }
        set
    }
    fn bitset_sum(set: u64) -> usize {
        (1..=52)
            .into_iter()
            .filter(move |n| (1 << n & set) != 0)
            .sum()
    }
    input
        .split(|b| *b == b'\n')
        .array_chunks::<3>()
        .fold((0, 0), |sum, l| {
            (
                sum.0
                    + l.iter()
                        .map(|l| {
                            let (left, right) = l.split_at(l.len() / 2);
                            bitset_sum(bitset(left) & bitset(right))
                        })
                        .sum::<usize>(),
                sum.1 + bitset_sum(bitset(l[0]) & bitset(l[1]) & bitset(l[2])),
            )
        })
}

fn day4(input: &str) -> (usize, usize) {
    input.split('\n').fold((0, 0), |count, l| {
        fn parse_range(s: &str) -> (u8, u8) {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }
        let (a, b) = l.split_once(',').unwrap();
        let (a, b) = (parse_range(a), parse_range(b));

        (
            count.0 + ((a.0 >= b.0 && a.1 <= b.1) || (a.0 <= b.0 && a.1 >= b.1)) as usize,
            count.1 + (a.1 >= b.0 && b.1 >= a.0) as usize,
        )
    })
}

fn day5(input: &str) -> (String, String) {
    let (state, cmds) = input.split_once("\n\n").unwrap();
    let mut simple: Vec<Vec<u8>> = {
        let mut lines = state.rsplit('\n');
        let count = lines.next().unwrap().len() / 4 + 1;
        let mut stacks: Vec<Vec<u8>> = vec![Vec::<u8>::new(); count];
        for l in lines {
            for (i, b) in l.as_bytes().iter().skip(1).step_by(4).enumerate() {
                if *b != b' ' {
                    stacks[i].push(*b);
                }
            }
        }
        stacks
    };
    let mut cmpx = simple.clone();

    for cmd in cmds.split('\n') {
        let mut words = cmd.split(' ');
        let amount = words.nth(1).unwrap().parse::<usize>().unwrap();
        let from = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = words.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let item = simple[from].pop().unwrap();
            simple[to].push(item);
        }

        let from_off = cmpx[from].len() - amount;
        let items: Vec<u8> = cmpx[from].drain(from_off..).collect();
        cmpx[to].extend(items);
    }
    fn fmt_stacks(stacks: &[Vec<u8>]) -> String {
        stacks
            .iter()
            .map(|s| std::char::from_u32(*s.last().unwrap() as u32).unwrap())
            .collect()
    }
    (fmt_stacks(&simple), fmt_stacks(&cmpx))
}

fn day6(str: &[u8]) -> (usize, usize) {
    fn find<const N: usize>(str: &[u8]) -> usize {
        str.windows(N)
            .position(|win| {
                let mut set: u32 = 0;
                win.iter().all(|c| {
                    let off = c - b'a';
                    set ^= 1 << off;
                    (1 << off & set) != 0
                })
            })
            .unwrap()
            + N
    }

    (find::<4>(str), find::<14>(str))
}

fn day7(str: &str) -> (usize, usize) {
    let mut lines = str.split('\n').peekable();
    let mut dirs = Vec::new();
    fn dir_size<'a>(
        lines: &mut Peekable<impl Iterator<Item = &'a str>>,
        dirs: &mut Vec<usize>,
    ) -> usize {
        lines.next(); // $ cd name
        lines.next(); // $ ls
        let mut size = 0;
        let mut subdir = 0;

        while let Some(l) = lines.peek() {
            if l.starts_with('$') {
                break;
            } else if l.strip_prefix("dir ").is_some() {
                subdir += 1;
            } else {
                size += l.split_once(' ').unwrap().0.parse::<usize>().unwrap()
            }
            lines.next();
        }

        for _ in 0..subdir {
            size += dir_size(lines, dirs);
        }

        lines.next(); // cd ..

        dirs.push(size);

        size
    }
    dir_size(&mut lines, &mut dirs);
    let unused_space = 70000000 - dirs.last().unwrap();
    let remove_space = 30000000 - unused_space;
    dirs.sort_unstable();
    (
        dirs.iter().filter(|size| (**size <= 100000)).sum(),
        *dirs.iter().find(|it| **it >= remove_space).unwrap(),
    )
}

fn day8(input: &[u8]) -> (usize, usize) {
    let width = input.iter().position(|b| *b == b'\n').unwrap();
    let height = input.split(|b| *b == b'\n').count();

    let at = |c, r| input[r * (width + 1) + c];

    let mut nb_visible = 0;
    let mut max = 0;

    for c in 0..width {
        for r in 0..height {
            if c == 0 || c == width - 1 || r == 0 || r == height - 1 {
                // Side are always visible from outside and have null scenic view
                nb_visible += 1;
            } else {
                let value = at(c, r);

                let left = (0..c).rev().position(|c| at(c, r) >= value);
                let right = (c + 1..width).position(|c| at(c, r) >= value);
                let top = (0..r).rev().position(|r| at(c, r) >= value);
                let btm = (r + 1..height).position(|r| at(c, r) >= value);

                if left.is_none() || right.is_none() || top.is_none() || btm.is_none() {
                    nb_visible += 1;
                }
                let scenic_view = (left.map(|it| it + 1).unwrap_or(c))
                    * (right.map(|it| it + 1).unwrap_or(width - c - 1))
                    * (top.map(|it| it + 1).unwrap_or(r))
                    * (btm.map(|it| it + 1).unwrap_or(height - r - 1));
                max = scenic_view.max(max);
            }
        }
    }
    (nb_visible, max)
}

fn day9(input: &str) -> (usize, usize) {
    let mut head = (0i16, 0i16);
    let mut knots = [head; 10];
    let mut rope2_set = BTreeSet::new();
    let mut rope10_set = BTreeSet::new();

    fn mov(tail: &mut (i16, i16), head: (i16, i16)) {
        let x = head.0 - tail.0;
        let y = head.1 - tail.1;
        if x.abs() > 1 || y.abs() > 1 {
            if x != 0 {
                tail.0 += x / x.abs();
            }

            if y != 0 {
                tail.1 += y / y.abs();
            }
        }
    }

    for l in input.split('\n') {
        let (dir, len) = (l.as_bytes()[0], l[2..].parse::<u8>().unwrap());
        for _ in 0..len {
            // Move head
            match dir {
                b'U' => head.1 += 1,
                b'L' => head.0 -= 1,
                b'R' => head.0 += 1,
                _ => head.1 -= 1,
            }

            // Move tail
            mov(&mut knots[8], head);
            for i in (0..8).rev() {
                let head = knots[i + 1];
                mov(&mut knots[i], head)
            }

            // Register pos
            rope2_set.insert(knots[8]);
            rope10_set.insert(knots[0]);
        }
    }
    (rope2_set.len(), rope10_set.len())
}

#[test]
fn test() {
    assert_eq!(day1(include_str!("../input/t01.txt")), (24000, 45000));
    assert_eq!(day2(include_bytes!("../input/t02.txt")), (15, 12));
    assert_eq!(day3(include_bytes!("../input/t03.txt")), (157, 70));
    assert_eq!(day4(include_str!("../input/t04.txt")), (2, 4));
    assert_eq!(
        day5(include_str!("../input/t05.txt")),
        ("CMZ".into(), "MCD".into())
    );
    assert_eq!(day7(include_str!("../input/t07.txt")), (95437, 24933642));
    assert_eq!(day8(include_bytes!("../input/t08.txt")), (21, 8));
    assert_eq!(day9(include_str!("../input/t09.txt")), (13, 1));
    assert_eq!(day9(include_str!("../input/t09_2.txt")), (88, 36));
}

fn main() {
    let (first, second) = day1(include_str!("../input/01.txt"));
    println!("Day1: {first} and {second}");
    let (first, second) = day2(include_bytes!("../input/02.txt"));
    println!("Day2: {first} and {second}");
    let (first, second) = day3(include_bytes!("../input/03.txt"));
    println!("Day3: {first} and {second}");
    let (first, second) = day4(include_str!("../input/04.txt"));
    println!("Day4: {first} and {second}");
    let (first, second) = day5(include_str!("../input/05.txt"));
    println!("Day5: {first} and {second}");
    let (first, second) = day6(include_bytes!("../input/06.txt"));
    println!("Day6: {first} and {second}");
    let (first, second) = day7(include_str!("../input/07.txt"));
    println!("Day7: {first} and {second}");
    let (first, second) = day8(include_bytes!("../input/08.txt"));
    println!("Day8: {first} and {second}");
    let (first, second) = day9(include_str!("../input/09.txt"));
    println!("Day9: {first} and {second}");
}

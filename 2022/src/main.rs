#![feature(slice_partition_dedup)]
#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![feature(iter_order_by)]

use std::{
    cmp::Ordering,
    collections::BTreeSet,
    iter::{empty, once, Peekable},
};

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

        let [f, t] = simple.get_many_mut([from, to]).unwrap();
        for _ in 0..amount {
            let item = f.pop().unwrap();
            t.push(item);
        }

        let [f, t] = cmpx.get_many_mut([from, to]).unwrap();
        t.extend(f.drain(f.len() - amount..));
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
        let mut set: u32 = 0;
        for i in 0..str.len() {
            set ^= 1 << (str[i] - b'a');
            if i >= N {
                set ^= 1 << (str[i - N] - b'a');
            }
            if (set.count_ones() as usize) == N {
                return i + 1;
            }
        }
        unreachable!()
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

fn day10(input: &str) -> (usize, String) {
    let mut reg = 1;
    let mut sum = 0;
    let mut cycle = 0;
    let mut screen = String::with_capacity(41 * 6);

    let mut tick = |reg| {
        cycle += 1;
        let off = cycle % 40;
        if off >= reg && off < reg + 3 {
            screen.push('#');
        } else {
            screen.push('.');
        }
        if off == 0 {
            screen.push('\n');
        }
        if (cycle - 20) % 40 == 0 {
            sum += reg * cycle;
        }
    };

    for cmd in input.split('\n') {
        if let Some((_, amount)) = cmd.split_once(' ') {
            // addx
            tick(reg);
            tick(reg);
            reg += amount.parse::<i16>().unwrap();
        } else {
            // noop
            tick(reg);
        }
    }
    (sum as usize, screen)
}

fn day11(input: &str) -> (usize, usize) {
    fn cmp<const N: usize, const DIV: u64>(input: &str) -> usize {
        enum Op {
            Add(u8),
            MulSelf,
            Mul(u8),
        }

        impl Op {
            pub fn apply(&self, value: u64) -> u64 {
                match self {
                    Op::Add(v) => value + *v as u64,
                    Op::MulSelf => value * value,
                    Op::Mul(v) => value * *v as u64,
                }
            }
        }

        struct Monkey {
            items: Vec<u64>,
            operation: Op,
            test: u8,
            if_true: u8,
            if_false: u8,
        }

        let mut monkeys: Vec<_> = input
            .split("\n\n")
            .map(|m| {
                let mut lines = m.split('\n').skip(1);
                let items = lines
                    .next()
                    .unwrap()
                    .strip_prefix("  Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|l| l.parse().unwrap())
                    .collect();
                let operation = {
                    let (op, value) = lines
                        .next()
                        .unwrap()
                        .strip_prefix("  Operation: new = old ")
                        .unwrap()
                        .split_once(' ')
                        .unwrap();
                    if op == "+" {
                        Op::Add(value.parse().unwrap())
                    } else if value == "old" {
                        Op::MulSelf
                    } else {
                        Op::Mul(value.parse().unwrap())
                    }
                };
                let sp =
                    |str: &str, prefix: &str| str.strip_prefix(prefix).unwrap().parse().unwrap();
                let test = sp(lines.next().unwrap(), "  Test: divisible by ");
                let if_true = sp(lines.next().unwrap(), "    If true: throw to monkey ");
                let if_false = sp(lines.next().unwrap(), "    If false: throw to monkey ");

                Monkey {
                    items,
                    operation,
                    test,
                    if_true,
                    if_false,
                }
            })
            .collect();
        let divisor: u64 = monkeys.iter().map(|m| m.test as u64).product();
        let mut counts: Vec<_> = monkeys.iter().map(|_| 0).collect();
        for _ in 0..N {
            for i in 0..monkeys.len() {
                let indices = [i, monkeys[i].if_true as usize, monkeys[i].if_false as usize];
                let [m, m_true, m_false] = monkeys.get_many_mut(indices).unwrap();
                counts[i] += m.items.len();
                for worry in m.items.drain(..) {
                    let worry = (m.operation.apply(worry) / DIV) % divisor;
                    if worry % m.test as u64 == 0 {
                        m_true.items.push(worry);
                    } else {
                        m_false.items.push(worry);
                    }
                }
            }
        }
        counts.sort_unstable();
        counts[counts.len() - 2..].iter().product()
    }
    (cmp::<20, 3>(input), cmp::<10000, 1>(input))
}

fn day12(input: &[u8]) -> (u16, u16) {
    let (grid, start, end, w) = {
        let mut grid = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut w = 0;

        for (r, l) in input.split(|b| *b == b'\n').enumerate() {
            w = l.len();
            for (c, b) in l.iter().enumerate() {
                grid.push(match b {
                    b'S' => {
                        start = (c, r);
                        b'a'
                    }
                    b'E' => {
                        end = (c, r);
                        b'z'
                    }
                    c => *c,
                })
            }
        }
        (grid, start, end, w)
    };

    fn dist_from(grid: &[u8], w: usize, start: (usize, usize), end: (usize, usize)) -> u16 {
        let mut visited = vec![false; grid.len()];
        let mut paths = vec![(start, 0)];
        loop {
            let Some(pos) = paths
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, dist))| dist)
                .map(|(i, _)| i) else {
                break u16::MAX;
            };
            let (pos, dist) = paths.swap_remove(pos);
            if pos == end {
                break dist;
            }
            let current = grid[pos.1 * w + pos.0];
            let new_paths = [(1, 0), (0, 1)]
                .into_iter()
                .flat_map(|(c, r)| {
                    [
                        (pos.0.wrapping_sub(c), pos.1.wrapping_sub(r)),
                        (pos.0 + c, pos.1 + r),
                    ]
                })
                // Filter positions outside of the grid
                .filter_map(|pos| {
                    ((pos.0 < w && pos.1 < grid.len() / w)
                        && grid[pos.1 * w + pos.0] <= current + 1
                        && !visited[pos.1 * w + pos.0])
                        .then(|| {
                            visited[pos.1 * w + pos.0] = true;
                            (pos, dist + 1)
                        })
                });
            paths.extend(new_paths);
        }
    }

    (
        dist_from(&grid, w, start, end),
        grid.iter()
            .enumerate()
            .filter_map(|(i, b)| (*b == b'a').then(|| dist_from(&grid, w, (i % w, i / w), end)))
            .min()
            .unwrap_or_default(),
    )
}

fn day13(input: &str) -> (usize, usize) {
    enum Ty<'a> {
        Nb(u8),
        Iter(Box<dyn Iterator<Item = Ty<'a>> + 'a>),
    }

    fn parse(str: &str) -> Ty<'_> {
        if str.is_empty() {
            Ty::Iter(Box::new(empty()))
        } else if str.starts_with('[') && str.ends_with(']') {
            let mut str = &str[1..str.len() - 1];
            Ty::Iter(Box::new(std::iter::from_fn(move || {
                if str.is_empty() {
                    None
                } else if str.starts_with('[') {
                    let mut skip = 0;
                    let pos = str
                        .find(|c| {
                            if c == '[' {
                                skip += 1;
                            } else if c == ']' {
                                skip -= 1;
                                if skip == 0 {
                                    return true;
                                }
                            }
                            false
                        })
                        .unwrap();
                    Some((&str[..pos + 1], &str[(pos + 2).min(str.len())..]))
                } else if let Some(pos) = str.find(',') {
                    Some((&str[..pos], &str[(pos + 1)..]))
                } else {
                    Some((str, ""))
                }
                .map(|(item, remain)| {
                    str = remain;
                    parse(item)
                })
            })))
        } else {
            Ty::Nb(str.parse().unwrap())
        }
    }

    fn ordered<'a>(a: Ty<'a>, b: Ty<'a>) -> Ordering {
        match (a, b) {
            (Ty::Nb(a), Ty::Nb(b)) => a.cmp(&b),
            (Ty::Nb(a), Ty::Iter(b)) => once(Ty::Nb(a)).cmp_by(b, ordered),
            (Ty::Iter(a), Ty::Nb(b)) => a.cmp_by(once(Ty::Nb(b)), ordered),
            (Ty::Iter(a), Ty::Iter(b)) => a.cmp_by(b, ordered),
        }
    }

    let dividers = ["[[2]]", "[[6]]"];
    let mut packets: Vec<_> = dividers.to_vec();
    let sum = input
        .split("\n\n")
        .enumerate()
        .map(|(i, group)| {
            let (first, second) = group.split_once('\n').unwrap();
            packets.extend([first, second].iter());
            match ordered(parse(first), parse(second)) {
                Ordering::Equal | Ordering::Less => i + 1,
                _ => 0,
            }
        })
        .sum();
    packets.sort_unstable_by(|a, b| ordered(parse(a), parse(b)));
    let key = dividers
        .iter()
        .map(|d| packets.iter().position(|it| it == d).unwrap() + 1)
        .product();
    (sum, key)
}

fn day14(input: &str) -> (usize, usize) {
    let paths = input.split('\n').map(|l| {
        l.split(" -> ").map(|c| {
            let (x, y) = c.split_once(',').unwrap();
            (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())
        })
    });

    // Parse dimensions

    let mut min_x = u16::MAX;
    let mut max_x = 0;
    let mut h = 0;

    for path in paths.clone() {
        for (x, y) in path {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            h = h.max(y + 1);
        }
    }

    // Init grid
    let w = (h + 2) * 2 + 1;
    min_x -= (w - (max_x - min_x)) / 2;
    let mut grid = vec![false; (w * (h + 1)) as usize];

    for path in paths {
        let mut from: Option<(u16, u16)> = None;
        for to in path {
            if let Some(from) = from {
                for x in from.0.min(to.0)..=from.0.max(to.0) {
                    for y in from.1.min(to.1)..=from.1.max(to.1) {
                        grid[(y * w + (x - min_x)) as usize] = true
                    }
                }
            }
            from = Some(to);
        }
    }

    fn sim(mut grid: Vec<bool>, min_x: u16, w: u16, h: u16, break_out: bool) -> usize {
        let mut count = 0;
        'sim: while !grid[(500 - min_x) as usize] {
            let mut pos = (500, 0);
            'mov: loop {
                let y = pos.1 + 1;
                let directions = [pos.0, pos.0 - 1, pos.0 + 1];
                for x in directions {
                    if y > h {
                        if break_out {
                            break 'sim;
                        } else {
                            break;
                        }
                    }
                    if !grid[(y * w + (x - min_x)) as usize] {
                        pos = (x, y);
                        continue 'mov;
                    }
                }
                count += 1;
                grid[(pos.1 * w + (pos.0 - min_x)) as usize] = true;
                // stuck
                break 'mov;
            }
        }
        count
    }

    // Simulate part 1
    (
        sim(grid.clone(), min_x, w, h, true),
        sim(grid, min_x, w, h, false),
    )
}

fn day15(y: i64, search: i64, input: &str) -> (usize, usize) {
    let (sensors, beacons) = input
        .split('\n')
        .fold((Vec::new(), BTreeSet::new()), |mut out, l| {
            let (sensor, beacon) = l.split_once(": ").unwrap();
            let sensor = sensor.strip_prefix("Sensor at ").unwrap();
            let beacon = beacon.strip_prefix("closest beacon is at ").unwrap();
            let pos_parser = |l: &str| {
                let (x, y) = l.strip_prefix("x=").unwrap().split_once(", y=").unwrap();
                (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
            };
            let sensor = pos_parser(sensor);
            let beacon = pos_parser(beacon);
            let dist = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
            out.0.push((sensor, dist));
            out.1.insert(beacon);
            out
        });

    fn intervals(y: i64, sensors: &[((i64, i64), u64)]) -> Vec<(i64, i64)> {
        let mut ints: Vec<_> = sensors
            .iter()
            .filter_map(|(s, dist)| {
                let diff = s.1.abs_diff(y);
                (diff < *dist).then(|| {
                    let diff = (dist - diff) as i64;
                    (s.0 - diff, s.0 + diff)
                })
            })
            .collect();
        ints.sort_unstable();
        let mut c = 0;
        for i in 1..ints.len() {
            if ints[c].1 >= ints[i].0 {
                ints[c].1 = ints[c].1.max(ints[i].1);
            } else {
                c += 1;
                ints[c] = ints[i];
            }
        }
        ints.truncate(c + 1);
        ints
    }

    let sum: i64 = intervals(y, &sensors)
        .into_iter()
        .map(|(s, e)| (e - s) + 1)
        .sum();
    let line = sum as usize - beacons.iter().filter(|it| it.1 == y).count();

    fn frequency(search: i64, sensors: &[((i64, i64), u64)]) -> usize {
        for y in 0..search + 1 {
            let ints = intervals(y, sensors);
            for (s, e) in ints {
                if s <= 0 && e > search {
                    break;
                } else if s <= 0 {
                    return ((e + 1) * 4000000 + y) as usize;
                }
            }
        }
        unreachable!()
    }
    (line, frequency(search, &sensors))
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
    assert_eq!(day10(include_str!("../input/t10.txt")).0, 13140);
    assert_eq!(day11(include_str!("../input/t11.txt")), (10605, 2713310158));
    assert_eq!(day12(include_bytes!("../input/t12.txt")), (31, 29));
    assert_eq!(day13(include_str!("../input/t13.txt")), (13, 140));
    assert_eq!(day14(include_str!("../input/t14.txt")), (24, 93));
    assert_eq!(
        day15(10, 20, include_str!("../input/t15.txt")),
        (26, 56000011)
    );
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
    let (first, second) = day10(include_str!("../input/10.txt"));
    println!("Day10: {first} and \n{second}");
    let (first, second) = day11(include_str!("../input/11.txt"));
    println!("Day11: {first} and {second}");
    let (first, second) = day12(include_bytes!("../input/12.txt"));
    println!("Day12: {first} and {second}");
    let (first, second) = day13(include_str!("../input/13.txt"));
    println!("Day13: {first} and {second}");
    let (first, second) = day14(include_str!("../input/14.txt"));
    println!("Day14: {first} and {second}");
    let (first, second) = day15(2000000, 4000000, include_str!("../input/15.txt"));
    println!("Day15: {first} and {second}");
}

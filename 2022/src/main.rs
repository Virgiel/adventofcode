#![feature(slice_partition_dedup)]
#![feature(iter_array_chunks)]

fn main() {
    let result = include_str!("../input/01.txt")
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
    println!("Day1: {} and {}", result[0], result.iter().sum::<usize>());

    let (win, strat) = include_str!("../input/02.txt")
        .lines()
        .fold((0, 0), |sum, l| {
            let opp = (l.as_bytes()[0] - b'A') as usize;
            let this = (l.as_bytes()[2] - b'X') as usize;
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
        });
    println!("Day2: {win} and {strat}");

    fn dedup(bs: &mut [u8]) -> impl Iterator<Item = &u8> {
        bs.sort_unstable();
        bs.partition_dedup().0.iter()
    }
    fn score_sum(iter: impl Iterator<Item = impl Iterator<Item = u8>>) -> usize {
        iter.map(|iter| {
            iter.map(|b| {
                (match b {
                    b'a'..=b'z' => b - b'a' + 1,
                    _ => b - b'A' + 27,
                }) as usize
            })
            .sum::<usize>()
        })
        .sum()
    }
    let sum_common: usize = score_sum(
        include_bytes!("../input/03.txt")
            .to_vec()
            .split_mut(|b| *b == b'\n')
            .map(|l| {
                let (left, right) = l.split_at_mut(l.len() / 2);
                let (mut left, mut right) = (dedup(left), dedup(right));
                std::iter::from_fn(move || {
                    let mut l = left.next();
                    let mut r = right.next();
                    loop {
                        if let (Some(lc), Some(rc)) = (&l, &r) {
                            if lc == rc {
                                return Some(**lc);
                            } else if lc < rc {
                                l = left.next()
                            } else {
                                r = right.next()
                            }
                        } else {
                            return None;
                        }
                    }
                })
            }),
    );
    let sum_common3 = score_sum(
        include_bytes!("../input/03.txt")
            .to_vec()
            .split_mut(|b| *b == b'\n')
            .array_chunks::<3>()
            .map(|l| {
                let [a, b, c] = l;
                let mut ai = dedup(a);
                let mut bi = dedup(b);
                let mut ci = dedup(c);
                std::iter::from_fn(move || {
                    let mut ac = ai.next();
                    let mut bc = bi.next();
                    let mut cc = ci.next();
                    loop {
                        if let (Some(a), Some(b), Some(c)) = (&ac, &bc, &cc) {
                            if a == b && b == c {
                                return Some(**a);
                            } else if a < b {
                                ac = ai.next()
                            } else if b < c {
                                bc = bi.next()
                            } else {
                                cc = ci.next()
                            }
                        } else {
                            return None;
                        }
                    }
                })
            }),
    );
    println!("Day3: {sum_common} and {sum_common3}");
    let (countain, overlap) =
        include_str!("../input/04.txt")
            .split("\n")
            .fold((0, 0), |count, l| {
                fn parse_range(s: &str) -> (u8, u8) {
                    let (a, b) = s.split_once("-").unwrap();
                    (a.parse().unwrap(), b.parse().unwrap())
                }
                let (a, b) = l.split_once(',').unwrap();
                let (a, b) = (parse_range(a), parse_range(b));

                (
                    count.0 + ((a.0 >= b.0 && a.1 <= b.1) || (a.0 <= b.0 && a.1 >= b.1)) as usize,
                    count.1 + (a.1 >= b.0 && b.1 >= a.0) as usize,
                )
            });
    println!("Day4: {countain} and {overlap}");

    let input = include_str!("../input/05.txt");
    let (state, cmds) = input.split_once("\n\n").unwrap();
    let mut simple: Vec<Vec<u8>> = {
        let mut lines = state.rsplit("\n");
        let count = lines.next().unwrap().len() / 4 + 1;
        let mut stacks: Vec<Vec<u8>> = vec![Vec::<u8>::new(); count];
        for l in lines {
            for (i, b) in l.as_bytes().into_iter().skip(1).step_by(4).enumerate() {
                if *b != b' ' {
                    stacks[i].push(*b);
                }
            }
        }
        stacks
    };
    let mut cmpx = simple.clone();

    for cmd in cmds.split("\n") {
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
            .into_iter()
            .map(|s| std::char::from_u32(*s.last().unwrap() as u32).unwrap())
            .collect()
    }
    println!("Day5: {} and {}", fmt_stacks(&simple), fmt_stacks(&cmpx))
}

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
}

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
}

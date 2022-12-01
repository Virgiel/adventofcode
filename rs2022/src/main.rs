fn main() {
    let input = include_str!("../input/01.txt");
    let nb_iter = input.split("\n\n").map(|l| {
        l.lines()
            .map(|s| s.parse::<usize>().unwrap())
            .sum::<usize>()
    });

    let mut state = [0usize; 3];
    for nb in nb_iter {
        if let Some(pos) = state.iter().position(|prev| *prev < nb) {
            for i in (pos..state.len() - 1).rev() {
                state[i + 1] = state[i]
            }
            state[pos] = nb
        }
    }
    println!("Day1: {} and {}", state[0], state.iter().sum::<usize>());
}

Object.defineProperty(Array.prototype, 'sum', {
  value: function () {
    return this.reduce((a, b) => a + b);
  },
});

{
  const result = Deno.readTextFileSync('input/01.txt')
    .split('\n\n')
    .map(s =>
      s
        .split('\n')
        .map(n => +n)
        .sum()
    )
    .sort((a, b) => -(a - b));

  console.log(`Day1: ${result[0]} and ${result.splice(0, 3).sum()}`);
}
{
  const [win, strat] = Deno.readTextFileSync('input/02.txt')
    .split('\n')
    .filter(l => l.length)
    .reduce(
      (sum, l) => {
        const [opp, self] = [
          l.charCodeAt(0) - 'A'.charCodeAt(),
          l.charCodeAt(2) - 'X'.charCodeAt(),
        ];
        const loose = [2, 0, 1][opp];
        const win = [1, 2, 0][opp];
        if (self == win) {
          sum[0] += 6 + self + 1;
        } else if (self == opp) {
          sum[0] += 3 + self + 1;
        } else {
          sum[0] += self + 1;
        }
        if (self == 0) {
          sum[1] += loose + 1;
        } else if (self == 1) {
          sum[1] += 3 + opp + 1;
        } else {
          sum[1] += 6 + win + 1;
        }
        return sum;
      },
      [0, 0]
    );
  console.log(`Day2: ${win} and ${strat}`);
}

Object.defineProperty(Array.prototype, 'sum', {
  value: function () {
    return this.reduce((a, b) => a + b);
  },
});

function* chunk(arr, n) {
  for (let i = 0; i < arr.length; i += n) {
    yield arr.slice(i, i + n);
  }
}

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

{
  const score = arr =>
    arr
      .map(b =>
        b >= 'a' && b <= 'z'
          ? b.charCodeAt() - 'a'.charCodeAt() + 1
          : b.charCodeAt() - 'A'.charCodeAt() + 27
      )
      .sum();
  const lines = Deno.readTextFileSync('input/03.txt').split('\n');
  const common = lines
    .map(l => {
      const [left, right] = [
        l.slice(0, l.length / 2),
        l.slice(l.length / 2),
      ].map(s => new Set(s));
      return score([...left].filter(i => right.has(i)));
    })
    .sum();
  const common3 = [...chunk(lines, 3)]
    .map(l => {
      const sets = l.map(s => new Set(s));
      return score([...sets[0]].filter(i => sets[1].has(i) && sets[2].has(i)));
    })
    .sum();
  console.log(`Day3: ${common} and ${common3}`);
}

{
  const [countain, overlap] = Deno.readTextFileSync('input/04.txt')
    .split('\n')
    .reduce(
      (sum, l) => {
        const [a, b] = l.split(',').map(l => l.split('-').map(n => +n));
        return [
          sum[0] +
            ((a[0] >= b[0] && a[1] <= b[1]) || (a[0] <= b[0] && a[1] >= b[1])),
          sum[1] + (a[1] >= b[0] && b[1] >= a[0]),
        ];
      },
      [0, 0]
    );
  console.log(`Day4: ${countain} and ${overlap}`);
}

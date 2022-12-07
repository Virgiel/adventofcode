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

function* range(start, end, step = 1) {
  for (let i = start; i < end; i += step) {
    yield i;
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

{
  const [state, cmds] = Deno.readTextFileSync('input/05.txt').split('\n\n');

  const parseState = () => {
    const lines = state.split('\n').reverse();
    const count = lines[0].length / 4 + 1;
    const stacks = Array.from({ length: count }, _ => []);
    for (const line of lines.splice(1)) {
      for (let i = 1; i < line.length; i += 4) {
        if (line[i] != ' ') {
          stacks[Math.floor(i / 4)].push(line[i]);
        }
      }
    }
    return stacks;
  };

  const simple = parseState(state);
  const cmpx = parseState(state);

  for (const cmd of cmds.split('\n')) {
    const words = cmd.split(' ');
    const amount = +words[1];
    const from = +words[3] - 1;
    const to = +words[5] - 1;

    for (let i = 0; i < amount; i++) {
      simple[to].push(simple[from].pop());
    }
    cmpx[to] = cmpx[to].concat(cmpx[from].splice(cmpx[from].length - amount));
  }
  const fmt = stacks => stacks.map(s => s[s.length - 1]).join('');

  console.log(`Day5: ${fmt(simple)} and ${fmt(cmpx)}`);
}

{
  const find = (str, n) => {
    for (const i of range(n, str.length)) {
      const slice = str.slice(i - n, i);
      const unique = slice
        .split('')
        .every(c => slice.split('').filter(it => it == c).length == 1);
      if (unique) {
        return i;
      }
    }
  };

  const input = Deno.readTextFileSync('input/06.txt');

  console.log(`Day6: ${find(input, 4)} and ${find(input, 14)}`);
}

{
  const lines = Deno.readTextFileSync('input/07.txt').split('\n').reverse();
  const dirs = [];

  const dir_size = (lines, dirs) => {
    lines.pop(); // $ cd name
    lines.pop(); // $ ls
    let size = 0;
    let subdir = 0;

    while (lines.length > 0 && !lines[lines.length - 1].startsWith('$')) {
      const cmd = lines.pop();
      if (cmd.startsWith('dir ')) {
        subdir++;
      } else {
        size += +cmd.split(' ')[0];
      }
    }

    for (; subdir > 0; subdir--) {
      size += dir_size(lines, dirs);
    }
    lines.pop();
    dirs.push(size);
    return size;
  };
  dir_size(lines, dirs);
  const unused_space = 70000000 - dirs[dirs.length - 1];
  const remove_space = 30000000 - unused_space;
  dirs.sort((a, b) => a - b);
  console.log(
    `Day6: ${dirs.filter(s => s <= 100000).sum()} and ${dirs.find(
      s => s >= remove_space
    )}`
  );
}

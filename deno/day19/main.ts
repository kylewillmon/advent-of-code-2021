type Beacon = [number, number, number];

function* allArrangements(b: Beacon): Generator<Beacon> {
  const [x, y, z] = b;
  for (
    const [a, b, c] of [
      [x, y, z],
      [x, -y, -z],
      [x, z, -y],
      [x, -z, y],
      [z, y, -x],
      [z, -y, x],
    ]
  ) {
    yield [a, b, c];
    yield [-a, -b, c];
    yield [-b, a, c];
    yield [b, -a, c];
  }
}

function addBeacon(a: Beacon, b: Beacon): Beacon {
  return [0, 1, 2].map((i) => a[i] + b[i]) as Beacon;
}

function subBeacon(a: Beacon, b: Beacon): Beacon {
  return [0, 1, 2].map((i) => a[i] - b[i]) as Beacon;
}

function eqBeacon(a: Beacon, b: Beacon): boolean {
  return a[0] == b[0] && a[1] == b[1] && a[2] == b[2];
}

class Scanner {
  public beacons: Beacon[];
  public loc: Beacon;
  constructor(public num: number) {
    this.beacons = [];
    this.loc = [0, 0, 0];
  }

  toArrangements(): Scanner[] {
    const a = Array.from({ length: 24 }, () => new Scanner(this.num));
    for (const b of this.beacons) {
      let i = 0;
      for (const nb of allArrangements(b)) {
        a[i++].beacons.push(nb);
      }
    }
    return a;
  }

  matches(s: Scanner): Beacon | null {
    for (const a of this.beacons) {
      for (const b of s.beacons) {
        const loc = subBeacon(a, b);
        const shared = s.beacons.map((b) => addBeacon(b, loc)).filter((b) =>
          this.beacons.some((x) => eqBeacon(b, x))
        );
        if (shared.length >= 12) return loc;
      }
    }
    return null;
  }
}

function parseInput(input: string): Scanner[] {
  return input.split("\n\n").filter((s) => s.trim()).map((s) => {
    const [header, ...lines] = s.trim().split("\n").filter((l) => l.trim());
    const num = Number(header.match(/scanner (\d+)/)![1]);
    const scanner = new Scanner(num);
    for (const b of lines) {
      scanner.beacons.push(b.split(",").map((v) => Number(v)) as Beacon);
    }
    return scanner;
  });
}

function findMatch(known: Scanner[], possible: Scanner[]): Scanner | null {
  for (const p of possible) {
    for (const k of known) {
      const loc = k.matches(p);
      if (loc) {
        p.loc = loc;
        for (let i = 0; i < p.beacons.length; i++) {
          p.beacons[i] = addBeacon(loc, p.beacons[i]);
        }
        return p;
      }
    }
  }
  return null;
}

export function solve(input: string): [number, number] {
  const [first, ...scanners] = parseInput(input);
  const solved = [first];
  let gen = [first];
  let possibles = scanners.map((s) => s.toArrangements());
  while (gen.length) {
    const nextGen: Scanner[] = [];
    const nextPos: Scanner[][] = [];
    for (const p of possibles) {
      const res = findMatch(gen, p);
      if (res) {
        nextGen.push(res);
      } else {
        nextPos.push(p);
      }
    }
    gen = nextGen;
    possibles = nextPos;

    for (const g of gen) solved.push(g);
  }

  const beacons: Set<string> = new Set();
  for (const s of solved) {
    for (const b of s.beacons) {
      beacons.add(b.join(","));
    }
  }

  let maxDist = 0;
  for (const a of solved) {
    for (const b of solved) {
      const dist = [0, 1, 2]
        .map((i) => Math.abs(a.loc[i] - b.loc[i]))
        .reduce((a, b) => a + b);
      maxDist = Math.max(maxDist, dist);
    }
  }
  return [beacons.size, maxDist];
}

if (import.meta.main) {
  const input = Deno.readTextFileSync(
    new URL(import.meta.resolve("./input.txt")),
  );
  const [p1, p2] = solve(input);
  console.log("Part 1: ", p1);
  console.log("Part 2: ", p2);
}

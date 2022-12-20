enum Amphipod {
  AMBER = 0,
  BRONZE,
  COPPER,
  DESERT,
}

const HALL_START = "..x.x.x.x..";

type Step = {
  dir: "HR" | "RH";
  from: number;
  to: number;
};

function toAmphipod(s: string): Amphipod {
  return s == "A"
    ? Amphipod.AMBER
    : s == "B"
    ? Amphipod.BRONZE
    : s == "C"
    ? Amphipod.COPPER
    : Amphipod.DESERT;
}

function ampToString(a: Amphipod): string {
  return String.fromCodePoint("A".codePointAt(0)! + a);
}

class State {
  constructor(
    public rooms: Amphipod[][],
    public hall: string,
    public energy: number,
  ) {}

  static fromInput(input: string): State {
    const amps = [...input.matchAll(/[A-D]/g)!].map((m) => toAmphipod(m[0]));
    return new State(
      [
        [amps[4], amps[0]],
        [amps[5], amps[1]],
        [amps[6], amps[2]],
        [amps[7], amps[3]],
      ],
      HALL_START,
      0,
    );
  }

  options(): Step[] {
    const steps: Step[] = [];
    return steps;
  }

  apply(s: Step): State {
    // TODO
    return this;
  }
}

export function part1(input: string): number {
  let gen: State[] = [State.fromInput(input)];
  console.log(gen);
  while (gen.length) {
    const nextGen: State[] = [];
    for (const s of gen) {
      for (const opt of s.options()) {
        nextGen.push(s.apply(opt));
      }
    }
    gen = nextGen;
  }
  return 0;
}

export function part2(input: string): number {
  return 0;
}

if (import.meta.main) {
  const input = Deno.readTextFileSync(
    new URL(import.meta.resolve("./input.txt")),
  );
  console.log("Part 1: ", part1(input));
  console.log("Part 2: ", part2(input));
}

enum Action {
  ON,
  OFF,
}

type Range = { min: number; max: number };
type Point = [number, number, number];

class Step {
  constructor(public action: Action, public ranges: [Range, Range, Range]) {}

  static fromLine(line: string): Step {
    const [, act, ...nums] = line.trim().match(
      /(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)/,
    )!;
    const ranges = [0, 2, 4].map((v) => ({
      min: Number(nums[v]),
      max: Number(nums[v + 1]),
    }));
    return new Step(
      act == "off" ? Action.OFF : Action.ON,
      ranges as [Range, Range, Range],
    );
  }

  clip(r: Range) {
    this.ranges.forEach((range) => {
      range.min = Math.max(range.min, r.min);
      range.max = Math.min(range.max, r.max);
    });
  }

  visit(f: (p: Point) => void) {
    const [xrange, yrange, zrange] = this.ranges;
    for (let x = xrange.min; x <= xrange.max; x++) {
      for (let y = yrange.min; y <= yrange.max; y++) {
        for (let z = zrange.min; z <= zrange.max; z++) {
          f([x, y, z]);
        }
      }
    }
  }
}

export function part1(input: string): number {
  const steps = input.split("\n").filter((l) => l.trim()).map(l => Step.fromLine(l));


  const cubes = new Set<string>();
  const on = (p: Point) => { cubes.add(p.join(',')); };
  const off = (p: Point) => { cubes.delete(p.join(',')); };

  for(const s of steps) {
    s.clip({ min: -50, max: 50 });
    s.visit(s.action == Action.ON ? on : off);
  }

  return cubes.size;
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

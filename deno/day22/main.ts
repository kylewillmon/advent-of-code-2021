enum Action {
  ON,
  OFF,
}

type Range = { start: number; end: number };
type Point = [number, number, number];

function rangeLen(a: Range): number {
  if (a.end <= a.start) return 0;
  return a.end - a.start;
}

function clipRange(a: Range, oth: Range): Range {
  return {
    start: Math.max(a.start, oth.start),
    end: Math.min(a.end, oth.end),
  };
}

function splitRange(a: Range, oth: Range): [Range | null, Range, Range | null] {
  const before = {
    start: a.start,
    end: Math.min(a.end, oth.start),
  };

  const inside = clipRange({ ...a }, oth);

  const after = {
    start: Math.max(a.start, oth.end),
    end: a.end,
  };

  const nullIfEmpty = (r: Range) => rangeLen(r) == 0 ? null : r;

  return [
    nullIfEmpty(before),
    inside,
    nullIfEmpty(after),
  ];
}

class Box {
  constructor(public ranges: [Range, Range, Range]) {}

  static fromLine(line: string): [Action, Box] {
    const [, act, ...nums] = line.trim().match(
      /(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)/,
    )!;
    const ranges = [0, 2, 4].map((v) => ({
      start: Number(nums[v]),
      end: Number(nums[v + 1]) + 1,
    }));
    return [
      act == "off" ? Action.OFF : Action.ON,
      new Box(
        ranges as [Range, Range, Range],
      ),
    ];
  }

  clip(r: Range) {
    this.ranges = this.ranges.map((range) => clipRange(range, r)) as [
      Range,
      Range,
      Range,
    ];
  }

  visit(f: (p: Point) => void) {
    const [xrange, yrange, zrange] = this.ranges;
    for (let x = xrange.start; x < xrange.end; x++) {
      for (let y = yrange.start; y < yrange.end; y++) {
        for (let z = zrange.start; z < zrange.end; z++) {
          f([x, y, z]);
        }
      }
    }
  }

  /** Split this box into sections that don't conflict with some other box */
  split(oth: Box): Box[] {
    if (
      this.ranges.some((r, i) =>
        rangeLen(clipRange({ ...r }, oth.ranges[i])) == 0
      )
    ) {
      return [this];
    }

    const boxes: Box[] = [];

    for (let i = 0; i < this.ranges.length; i++) {
      const [before, inside, after] = splitRange(this.ranges[i], oth.ranges[i]);
      if (before) {
        const ranges = [...this.ranges];
        ranges[i] = before;
        boxes.push(new Box(ranges as [Range, Range, Range]));
      }
      if (after) {
        const ranges = [...this.ranges];
        ranges[i] = after;
        boxes.push(new Box(ranges as [Range, Range, Range]));
      }
      this.ranges[i] = inside;
    }

    return boxes;
  }

  size(): number {
    return this.ranges.map((r) => rangeLen(r)).reduce((a, b) => a * b);
  }
}

export function part1(input: string): number {
  const steps = input.split("\n").filter((l) => l.trim()).map((l) =>
    Box.fromLine(l)
  );

  let boxes: Box[] = [];

  for (const [act, box] of steps) {
    box.clip({ start: -50, end: 51 });
    if (box.size() == 0) continue;

    boxes = boxes.flatMap((b) => b.split(box));

    if (act == Action.ON) {
      boxes.push(box);
    }
  }

  return boxes.map((b) => b.size()).reduce((a, b) => a + b);
}

export function part2(input: string): number {
  const steps = input.split("\n").filter((l) => l.trim()).map((l) =>
    Box.fromLine(l)
  );

  let boxes: Box[] = [];

  for (const [act, box] of steps) {
    if (box.size() == 0) continue;

    boxes = boxes.flatMap((b) => b.split(box));

    if (act == Action.ON) {
      boxes.push(box);
    }
  }

  return boxes.map((b) => b.size()).reduce((a, b) => a + b);
}

if (import.meta.main) {
  const input = Deno.readTextFileSync(
    new URL(import.meta.resolve("./input.txt")),
  );
  console.log("Part 1: ", part1(input));
  console.log("Part 2: ", part2(input));
}

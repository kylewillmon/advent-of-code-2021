class SeaFloor {
  constructor(public g: string[][]) {}

  moved(cuc: string, r: number, c: number): SeaFloor {
    const g: string[][] = [];
    for (let i = 0; i < this.g.length; i++) {
      const row: string[] = [];
      for (let j = 0; j < this.g[i].length; j++) {
        let val = this.g[i][j];
        if (val == cuc) val = ".";
        row.push(val);
      }
      g.push(row);
    }

    for (let i = 0; i < this.g.length; i++) {
      for (let j = 0; j < this.g[i].length; j++) {
        if (this.g[i][j] != cuc) continue;
        const [ni, nj] = [(i + r) % this.g.length, (c + j) % this.g[0].length];
        if (this.g[ni][nj] == ".") {
          g[ni][nj] = cuc;
        } else {
          g[i][j] = cuc;
        }
      }
    }
    return new SeaFloor(g);
  }

  print() {
    for (const r of this.g) {
      console.log(r.join(""));
    }
    console.log();
  }

  step(): SeaFloor {
    return this.moved(">", 0, 1).moved("v", 1, 0);
  }

  equals(other: SeaFloor): boolean {
    for (let i = 0; i < this.g.length; i++) {
      for (let j = 0; j < this.g[i].length; j++) {
        if (this.g[i][j] != other.g[i][j]) return false;
      }
    }
    return true;
  }
}

function parse(input: string): SeaFloor {
  return new SeaFloor(
    input
      .split("\n")
      .map((l) => l.trim())
      .filter((l) => l).map((l) => Array.from(l)),
  );
}

export function part1(input: string): number {
  let floor = parse(input);
  let count = 0;
  do {
    const next = floor.step();
    if (next.equals(floor)) {
      break;
    }
    floor = next;
  } while (++count);
  return count + 1;
}

if (import.meta.main) {
  const input = Deno.readTextFileSync(
    new URL(import.meta.resolve("./input.txt")),
  );
  console.log("Part 1: ", part1(input));
}

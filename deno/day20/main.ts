enum Pixel {
  LIGHT,
  DARK,
}

class Image {
  public g: Pixel[][];
  public border: Pixel;
  constructor(g: string[], border: Pixel) {
    this.g = g.map((r) =>
      Array.from(r, (c) => c == "#" ? Pixel.LIGHT : Pixel.DARK)
    );
    this.border = border;
  }

  print() {
    for (const r of this.g) {
      console.log(r.map((p) => p == Pixel.LIGHT ? "#" : ".").join(""));
    }
    console.log();
  }

  get(r: number, c: number) {
    if (r < 0 || r >= this.g.length) return this.border;
    const row = this.g[r];
    if (c < 0 || c >= row.length) return this.border;
    return row[c];
  }

  enhance(map: string): Image {
    const g: string[] = [];
    for (let r = -2; r < this.g.length; r++) {
      let row = "";
      for (let c = -2; c < this.g[0].length; c++) {
        let val = 0;
        for (let i = 0; i < 3; i++) {
          for (let j = 0; j < 3; j++) {
            val *= 2;
            if (this.get(r + i, c + j) == Pixel.LIGHT) val += 1;
          }
        }
        row += map[val];
      }
      g.push(row);
    }
    const val = this.border == Pixel.LIGHT ? 511 : 0;
    return new Image(g, map[val] == "#" ? Pixel.LIGHT : Pixel.DARK);
  }
}

function parse(input: string): [string, Image] {
  let [map, img] = input.split("\n\n").filter((s) => s.trim());
  map = map.split("\n").join("").trim();
  return [map, new Image(img.split("\n").filter((l) => l.trim()), Pixel.DARK)];
}

export function part1(input: string): number {
  let [map, img] = parse(input);
  img = img.enhance(map);
  img = img.enhance(map);
  return img.g.reduce((acc, row) => (
    acc + row.reduce((a, p) => p == Pixel.LIGHT ? a + 1 : a, 0)
  ), 0);
}

export function part2(input: string): number {
  let [map, img] = parse(input);
  for (let i = 0; i < 50; i++) {
    img = img.enhance(map);
  }
  return img.g.reduce((acc, row) => (
    acc + row.reduce((a, p) => p == Pixel.LIGHT ? a + 1 : a, 0)
  ), 0);
}

if (import.meta.main) {
  const input = Deno.readTextFileSync(
    new URL(import.meta.resolve("./input.txt")),
  );
  console.log("Part 1: ", part1(input));
  console.log("Part 2: ", part2(input));
}

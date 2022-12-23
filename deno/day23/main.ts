enum Amphipod {
  AMBER = 0,
  BRONZE,
  COPPER,
  DESERT,
}

const HALL_START = [
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
  null,
];

type Step = {
  dir: "HR" | "RH";
  hall: number;
  room: number;
};

const ampParse = (s: string) =>
  (s.codePointAt(0)! - "A".codePointAt(0)!) as Amphipod;
const ampToString = (a: Amphipod) =>
  String.fromCodePoint("A".codePointAt(0)! + a);
const ampEnergy = (a: Amphipod) => Math.pow(10, a);
const isRoomSquare = (idx: number) =>
  idx == 2 || idx == 4 || idx == 6 || idx == 8;

class State {
  constructor(
    public rooms: Amphipod[][],
    public hall: (Amphipod | null)[],
    public energy: number,
    public room_size: number,
  ) {}

  static fromInput(input: string): State {
    const amps = [...input.matchAll(/[A-D]/g)!].map((m) => ampParse(m[0]));
    return new State(
      [
        [amps[4], amps[0]],
        [amps[5], amps[1]],
        [amps[6], amps[2]],
        [amps[7], amps[3]],
      ],
      [...HALL_START],
      0,
      2,
    );
  }

  isSolved(): boolean {
    return this.rooms.every((r, a) =>
      r.length == this.room_size && r.every((r) => r == a)
    );
  }

  print() {
    console.log("#".repeat(13));
    console.log(
      ["#", ...this.hall.map((h) => h === null ? "." : ampToString(h)), "#"]
        .join(""),
    );
    for (let i = this.room_size - 1; i >= 0; i--) {
      console.log(
        "  #" +
          this.rooms.map((r) => i < r.length ? ampToString(r[i]) : ".").join(
            "#",
          ) + "#",
      );
    }
    console.log("  " + "#".repeat(9));
  }

  options(): Step[] {
    const steps: Step[] = [];
    // Go home
    for (let r = 0; r < this.rooms.length; r++) {
      // Ignore rooms with strangers in them
      if (this.rooms[r].some((a) => a != r)) continue;

      for (let sq = r * 2 + 2; sq >= 0; sq--) {
        if (this.hall[sq] != null) {
          if (this.hall[sq] == r) {
            return [{
              dir: "HR",
              room: r,
              hall: sq,
            }];
          }
          break;
        }
      }
      for (let sq = r * 2 + 2; sq < this.hall.length; sq++) {
        if (this.hall[sq] != null) {
          if (this.hall[sq] == r) {
            return [{
              dir: "HR",
              room: r,
              hall: sq,
            }];
          }
          break;
        }
      }
    }

    let rooms = [0, 1, 2, 3].filter((r) => {
      // Ignore empty rooms
      if (!this.rooms[r].length) return false;
      // Ignore solved rooms
      if (this.rooms[r].every((a) => a == r)) return false;
      return true;
    });

    //const half_empty = rooms.filter((r) =>
    //  this.rooms[r].length != this.room_size
    //);
    //if (half_empty.length != 0) rooms = half_empty;

    // Try to leave rooms
    for (const r of rooms) {
      const dests: number[] = [];
      for (let sq = r * 2 + 2; sq >= 0; sq--) {
        if (isRoomSquare(sq)) continue;
        if (this.hall[sq] != null) break;
        dests.push(sq);
      }
      for (let sq = r * 2 + 2; sq < this.hall.length; sq++) {
        if (isRoomSquare(sq)) continue;
        if (this.hall[sq] != null) break;
        dests.push(sq);
      }

      for (const d of dests) {
        steps.push({ dir: "RH", room: r, hall: d });
      }
    }
    return steps;
  }

  apply(s: Step): State {
    const newState = new State(
      [...this.rooms.map((r) => [...r])],
      [...this.hall],
      this.energy,
      this.room_size,
    );

    const room = newState.rooms[s.room];
    let dist = Math.abs((s.room * 2 + 2) - s.hall);
    dist += this.room_size - room.length;
    let amp: Amphipod;
    if (s.dir == "HR") {
      amp = newState.hall[s.hall]!;
      room.push(amp);
      newState.hall[s.hall] = null;
    } else {
      dist++;
      amp = room.pop()!;
      newState.hall[s.hall] = amp;
    }
    newState.energy += ampEnergy(amp) * dist;
    return newState;
  }
}

export function part1(input: string): number {
  let gen: State[] = [State.fromInput(input)];
  let min = Number.POSITIVE_INFINITY;
  while (gen.length) {
    const nextGen: State[] = [];
    for (const s of gen) {
      for (const opt of s.options()) {
        const newState = s.apply(opt);
        if (newState.isSolved()) {
          min = Math.min(min, newState.energy);
        } else if (newState.energy < min) {
          nextGen.push(newState);
        }
      }
    }
    console.log(nextGen.length);
    gen = nextGen;
  }
  return min;
}

export function part2(input: string): number {
  const state = State.fromInput(input);
  state.room_size = 4;
  state.rooms[0].splice(1, 0, Amphipod.DESERT, Amphipod.DESERT);
  state.rooms[1].splice(1, 0, Amphipod.BRONZE, Amphipod.COPPER);
  state.rooms[2].splice(1, 0, Amphipod.AMBER, Amphipod.BRONZE);
  state.rooms[3].splice(1, 0, Amphipod.COPPER, Amphipod.AMBER);

  state.print();

  let gen: State[] = [state];
  let min = Number.POSITIVE_INFINITY;
  while (gen.length) {
    const nextGen: State[] = [];
    for (const s of gen) {
      for (const opt of s.options()) {
        const newState = s.apply(opt);
        if (newState.isSolved()) {
          min = Math.min(min, newState.energy);
        } else if (newState.energy < min) {
          nextGen.push(newState);
        }
      }
    }
    console.log(nextGen.length);
    gen = nextGen;
  }
  return min;
}

if (import.meta.main) {
  const input = Deno.readTextFileSync(
    new URL(import.meta.resolve("./input.txt")),
  );
  console.log("Part 1: ", part1(input));
  console.log("Part 2: ", part2(input));
}

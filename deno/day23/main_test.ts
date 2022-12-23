import { assertEquals } from "https://deno.land/std@0.166.0/testing/asserts.ts";
import { part1, part2 } from "./main.ts";

const example = `
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
`;

Deno.test("example", () => {
  assertEquals(part1(example), 12521);
  assertEquals(part2(example), 44169);
});


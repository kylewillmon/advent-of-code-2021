import { assertEquals } from "https://deno.land/std@0.166.0/testing/asserts.ts";
import { part1 } from "./main.ts";

const example = `v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>`;

Deno.test(function day25Test() {
  assertEquals(part1(example), 58);
});

use std::collections::{ HashSet };
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> HashSet<i32> {
  input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve1(numbers: &HashSet<i32>) -> i32 {
  for a in numbers.iter() {
    let b = 2020 - a;
    if numbers.contains(&b) {
      return a * b;
    }
  }
  return 0;
}

#[aoc(day1, part2)]
pub fn solve2(numbers: &HashSet<i32>) -> i32 {
  for (a, b, c) in numbers.iter().tuple_combinations() {
    if a + b + c == 2020 {
      return a * b * c;
    }
  }
  return 0;
}
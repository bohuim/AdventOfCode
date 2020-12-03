use regex::Regex;
use std::fmt;

//#mark - Types 

pub struct Entry {
  a: usize,
  b: usize,
  ch: char,
  pw: String,
}

impl fmt::Display for Entry {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}-{} {}: {}", self.a, self.b, self.ch, self.pw)
  }
}

//#mark - Solution

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Entry> {
  let matcher = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z0-9]+)$").unwrap();
  input.lines()
    .map(|line| {
      // Map each line into an Entry.
      let groups = matcher.captures(line).unwrap();
      Entry {
        a: groups.get(1).unwrap().as_str().parse().unwrap(),
        b: groups.get(2).unwrap().as_str().parse().unwrap(),
        ch: groups.get(3).unwrap().as_str().chars().next().unwrap(),
        pw: groups.get(4).unwrap().as_str().to_string(),
      }
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn solve1(entries: &Vec<Entry>) -> usize {
  entries.iter()
    .filter(|entry| {
      // Count number of character occurences.
      let count = entry.pw.chars().filter(|c| c == &entry.ch).count();
      // Count must be within range [a, b] inclusive.
      (entry.a ..= entry.b).contains(&count)
    })
    .count()
}

#[aoc(day2, part2)]
pub fn solve2(entries: &Vec<Entry>) -> usize {
  entries.iter()
    // Filter for valid entries.
    .filter(|entry| {
      // Valid condition for each entry: # of positional & character match must be 1.
      entry.pw
        .char_indices()
        .filter(|tup| { // (index, character)
          let p = tup.0 + 1;
          let c = tup.1;
          (p == entry.a || p == entry.b) && (c == entry.ch)
        })
        .count() == 1
    })
    // Number of valid entries.
    .count()
}
use std::collections::HashSet;

const ANY_CHARS: &str = "";
const ALL_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

#[aoc(day6, part1)]
pub fn solve1(input: &str) -> usize {
  count_forms(true, input)
}

#[aoc(day6, part2)]
pub fn solve2(input: &str) -> usize {
  count_forms(false, input)
}

//#mark - Helpers

/// Counts the number of yes answers, depending on "anyone" or "everyone".
/// 
/// # parameters:
/// - `any`: whether to use union if true, intersection otherwise.
/// - `input`: str of the entire input.
fn count_forms(any: bool, input: &str) -> usize {
  input.split("\n\n").map(|group| form(any, group)).map(|form| form.len()).sum()
}

/// Creates a `HashSet<char>` reprensting a group customs form.
/// 
/// # parameters:
/// - `any`: whether to use union if true, intersection otherwise.
/// - `group`: str representing a group with each person's answers per line.
fn form(any: bool, group: &str) -> HashSet<char> {
  let charset = if any { ANY_CHARS } else { ALL_CHARS };
  group.trim().lines()
    // map each line to set of chars
    .map(|line| line.trim().chars().collect())
    // reduce into a single set
    .fold(charset.chars().collect(), |acc, set| 
      if any { &acc | &set } else { &acc & &set })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn form_any() {
    assert_eq!(
      form(true, r"
        a
        bc
        ac"), 
      "abc".chars().collect());
    assert_eq!(
      form(true, r"
        a
        abc
        ac"), 
      "abc".chars().collect());
  }

  #[test]
  fn form_all() {
    assert_eq!(
      form(false, r"
        a
        bc
        ac"), 
      "".chars().collect());
    assert_eq!(
      form(false, r"
        a
        abc
        ac"), 
      "a".chars().collect());
  }
}
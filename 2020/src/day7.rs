use lazy_static::lazy_static;j
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
  static ref RE_BAG0: Regex = Regex::new(r"^(?<name>[a-z ]+) bags contain no other bags\.$").unwrap();
  static ref RE_BAG1: Regex = Regex::new(r"^(?<name>[a-z ]+) bags contain (?<count>\d+) (?<color>[a-z ]+) bags?\.$").unwrap();
  static ref RE_BAG2: Regex = Regex::new(r"^(?<name>[a-z ]+) bags contain (?<count1>\d+) (?<color1>[a-z ]+) bags?, (?<count2>\d+) (?<color2>[a-z ]+) bags?\.$").unwrap();
}

///
pub struct Bag {
  a: Option<SubBag>,
  b: Option<SubBag>,
}
impl Bag {
  ///
  fn from_str(s: &str) -> Bag {

    Bag {}
  }
}

/// Reference to a sub-bag name and size.
pub struct SubBag {
  color: String,
  count: usize,
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> HashMap<String, Bag> {
  input.lines().map(Bag::from_str).collect()
}

#[aoc(day7)]
pub fn solve1(_bags: &HashMap<String, Bag>) -> usize {
  42
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bag_from_str() {

  }
}
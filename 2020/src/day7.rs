use core::ops::Index;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";
lazy_static! {
  static ref RE_OUTER: Regex = Regex::new(r"(?P<name>[a-z ]+) bags contain").unwrap();
  static ref RE_INNER: Regex = Regex::new(r"(?P<size>\d+) (?P<name>[a-z ]+) bag").unwrap();
}

/// Catalog of all bags.
pub struct Bags {
  entries: HashMap<String, OuterBag>
}
impl Index<&str> for Bags {
  type Output = OuterBag;

  fn index(&self, i: &str) -> &Self::Output {
    // Note: this throws if i doesn't exist.
    &self.entries[i]
  }
}
impl Bags {
  /// Checks whether bag named `a` eventually contains bag named `b`.
  fn path_exists(&self, a: &str, b: &str) -> bool {
    self[a].inners.iter()
      .any(|inner| {
        inner.name == b || self.path_exists(&inner.name, b)
      })
  }

  /// Counts the number of inner children bags this bag contains.
  fn nested_size_of(&self, name: &str) -> usize {
    self[name].inners.iter()
      .map(|inner| {
        (inner.size, self.nested_size_of(&inner.name))
      })
      .fold(0, |acc, (n, x)| acc + n + n * x)
  }
}

/// Bag representation as a list of `InnerBag` references it holds.
#[derive(PartialEq, Eq, Debug)]
pub struct OuterBag {
  inners: Vec<InnerBag>
}
impl OuterBag {
  /// Parses a str line into a outer bag's name and `OuterBag` instance.
  fn parse(s: &str) -> (String, OuterBag) {
    let name = RE_OUTER.captures(s).unwrap().name("name").unwrap().as_str().trim().to_string();
    let inners: Vec<InnerBag> = RE_INNER.captures_iter(s)
      .map(|captures| InnerBag {
        name: captures.name("name").unwrap().as_str().trim().to_string(),
        size: captures.name("size").unwrap().as_str().trim().parse().unwrap(),
      })
      .collect();
    (name, OuterBag { inners })
  }
}

/// Reference to a inner bag name and size.
#[derive(PartialEq, Eq, Debug)]
pub struct InnerBag {
  name: String,
  size: usize,
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Bags {
  Bags {
    entries: input.trim().lines().map(OuterBag::parse).collect()
  }
}

#[aoc(day7, part1)]
pub fn solve1(bags: &Bags) -> usize {
  bags.entries.keys().filter(|a| bags.path_exists(a, SHINY_GOLD)).count()
}

#[aoc(day7, part2)]
pub fn solve2(bags: &Bags) -> usize {
  bags.nested_size_of(SHINY_GOLD)
}

#[cfg(test)]
mod tests {
  use super::*;

  lazy_static! {
    static ref BAGS: Bags = parse(r"
      light red bags contain 1 bright white bag, 2 muted yellow bags.
      dark orange bags contain 3 bright white bags, 4 muted yellow bags.
      bright white bags contain 1 shiny gold bag.
      muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
      dark olive bags contain 3 faded blue bags, 4 dotted black bags.
      vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
      faded blue bags contain no other bags.
      dotted black bags contain no other bags.
    ");
  }

  #[test]
  fn parse_bag_0() {
    let (name, outer) = OuterBag::parse("shiny gold bags contain no other bags.");
    assert_eq!(name, "shiny gold");
    assert_eq!(outer, OuterBag { inners: vec![] });
  }

  #[test]
  fn parse_bag_1() {
    let (name, outer) = OuterBag::parse("bright white bags contain 1 shiny gold bag.");
    assert_eq!(name, "bright white");
    assert_eq!(outer, OuterBag {
      inners: vec![
        InnerBag { name: "shiny gold".to_string(), size: 1 }
      ]
    });
  }

  #[test]
  fn parse_bag_2() {
    let (name, outer) = OuterBag::parse("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.");
    assert_eq!(name, "vibrant plum");
    assert_eq!(outer, OuterBag {
      inners: vec![
        InnerBag { name: "faded blue".to_string(), size: 5 },
        InnerBag { name: "dotted black".to_string(), size: 6 },
      ]
    });
  }

  #[test]
  fn bags_path_exists() {
    assert!( BAGS.path_exists("bright white", "shiny gold"));
    assert!( BAGS.path_exists("muted yellow", "shiny gold"));
    assert!( BAGS.path_exists("dark orange", "shiny gold"));
    assert!( BAGS.path_exists("light red", "shiny gold"));
    assert!(!BAGS.path_exists("vibrant plum", "shiny gold"));
    assert!(!BAGS.path_exists("shiny gold", "shiny gold"));
    assert!(!BAGS.path_exists("shiny gold", "shiny gold"));
    assert_eq!(solve1(&BAGS), 4);
  }

   #[test]
  fn bags_nested_size_of() {
    assert_eq!(BAGS.nested_size_of("dotted black"), 0);
    assert_eq!(BAGS.nested_size_of("faded blue"), 0);
    assert_eq!(BAGS.nested_size_of("vibrant plum"), 11);
    assert_eq!(BAGS.nested_size_of("dark olive"), 7);
    assert_eq!(solve2(&BAGS), 32);
  }
}
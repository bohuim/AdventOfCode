use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub type Edges = HashMap<String, Edge>;
pub type Dists = HashMap<String, u32>;

#[aoc_generator(day6)]
pub fn parse(input: &str) -> (Edges, Dists) {
  let mut dists = HashMap::new();
  dists.insert("COM".to_string(), 0);

  let edges = parse_edges(input);
  for edge in edges.values() {
    calc_count(&edge.orbiter, &edges, &mut dists);
  }
  (edges, dists)
}

#[aoc(day6, part1)]
pub fn part1(input: &(Edges, Dists)) -> u32 {
  // Sum all orbital edge distances.
  input.1.values().sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &(Edges, Dists)) -> usize {
  // Get path from YOU & SAN to COM.
  let you_path = path_to_com("YOU", &input.0);
  let san_path = path_to_com("SAN", &input.0);

  // Find the common nodes.
  let you_set: HashSet<&str> = HashSet::from_iter(you_path.iter().map(|s| *s));
  let san_set: HashSet<&str> = HashSet::from_iter(san_path.iter().map(|s| *s));
  let common = you_set.intersection(&san_set);

  // Find the closest common node N.
  let (node, dist) = common
    .map(|&x| {
      let index = you_path.iter().position(|&y| x == y).unwrap();
      (x, index)
    })
    .min_by_key(|t| t.1)
    .unwrap();

  // Add dist from YOU -> N and SAN -> N.
  dist + san_path.iter().position(|&x| x == node).unwrap()
}

fn parse_edges(input: &str) -> Edges {
  let mut edges = HashMap::new();
  for pair in input.lines() {
    let part: Vec<&str> = pair.splitn(2, ')').collect();
    let edge = Edge::new(part[0], part[1]);
    edges.insert(edge.orbiter.clone(), edge);
  }
  edges // ownership transferred out
}

fn calc_count(node: &str, edges: &Edges, dists: &mut Dists) -> u32 {
  match dists.get(node) {
    Some(count) => *count,
    None => {
      let count = 1 + calc_count(&edges.get(node).unwrap().orbitee, &edges, dists);
      dists.insert(node.to_string(), count);
      count
    }
  }
}

// Path to COM excluding the specified node and COM.
fn path_to_com<'a>(node: &str, edges: &'a Edges) -> Vec<&'a str> {
  let mut path: Vec<&str> = Vec::new();
  let mut curr = edges.get(node).unwrap();
  while curr.orbitee != "COM" {
    path.push(&curr.orbitee);
    curr = edges.get(&curr.orbitee).unwrap();
  }
  // path.push("COM");
  path
}

// DEFINITIONS -----
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Edge {
  orbiter: String,
  orbitee: String,
}
impl Edge {
  fn new(orbitee: &str, orbiter: &str) -> Self {
    Edge {
      orbiter: orbiter.to_string(),
      orbitee: orbitee.to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test6p1_input() {
    parse(&"\
      A)B
      B)C
      C)D");
  }
}

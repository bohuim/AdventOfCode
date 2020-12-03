use std::collections::HashSet;

type Grid = Vec<HashSet<usize>>;
pub struct Road {
  width: usize,
  value: Grid,
}

//#mark - Solution

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Road {
  let grid: Grid = input.lines()
    .map(|line|
      // For each line, collect tree indices as a set.
      line.char_indices()
        .filter_map(|(i, c)| if c == '#' { Some(i) } else { None })
        .collect()
    )
    .collect();
  return Road {
    width: input.lines().next().unwrap().len(), 
    value: grid
  };
}

#[aoc(day3, part1)]
pub fn solve1(road: &Road) -> usize {
  // For each row, if there's a tree at position, increment the hit count.
  // Add right over 3 positions, wrapping around on width overflow.
  let mut hit: usize = 0;
  let mut pos: usize = 0;
  for row in &road.value {
    if row.contains(&pos) { hit += 1; }
    pos = (pos + 3) % road.width
  }
  hit
}
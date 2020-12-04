use std::collections::HashSet;

type Grid = Vec<HashSet<usize>>;
pub struct Road {
  width: usize,
  value: Grid,
}

pub struct Slope {
  right: usize,
  down: usize,
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
  calculate_hits(road, &Slope { right: 3, down: 1})
}

#[aoc(day3, part2)]
pub fn solve2(road: &Road) -> usize {
  let slopes = vec![
    Slope { right: 1, down: 1 },
    Slope { right: 3, down: 1 },
    Slope { right: 5, down: 1 },
    Slope { right: 7, down: 1 },
    Slope { right: 1, down: 2 },
  ];
  slopes.iter()
    .map(|slope| calculate_hits(road, slope))
    .product()
}

//#mark - Helpers

/// Calculates the number of trees hit on `road` when going at `slope`.
/// 
/// Starting at the top left, increment hit count if there's a tree at the current position, 
/// then go down-right by the slope, wrapping around to the left on road width overflow.
fn calculate_hits(road: &Road, slope: &Slope) -> usize {
  let mut hit: usize = 0;
  let mut row: usize = 0;
  let mut pos: usize = 0;
  while row < road.value.len() {
    if road.value[row].contains(&pos) { hit += 1 }
    pos = (pos + slope.right) % road.width;
    row += slope.down;
  }
  hit
}

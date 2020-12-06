use core::ops::Range;

#[derive(PartialEq, Eq, Debug)]
pub struct Seat {
  row: usize,
  col: usize,
  sid: usize,
}
impl Seat {
  /// Designated initializer from a 10 character token.
  fn from_str(s: &str) -> Seat {
    let reducer = |rng: Range<usize>, first: bool| -> Range<usize> {
      let mid = rng.start + (rng.end - rng.start) / 2;
      if first { rng.start..mid } else { mid..rng.end }
    };
    let &row = &s[..8].chars()
      .filter_map(|ch| match ch {
        'F' => Some(true),
        'B' => Some(false),
        _ => None,
      })
      .fold(0..128, reducer)
      .start;
    let &col = &s[0..].chars()
      .filter_map(|ch| match ch {
        'L' => Some(true),
        'R' => Some(false),
        _ => None,
      })
      .fold(0..8, reducer)
      .start;
    Seat { row, col, sid: (8 * row + col) }
  }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vec<Seat> {
  input.lines().map(Seat::from_str).collect()
}

#[aoc(day5, part1)]
pub fn solve1(seats: &Vec<Seat>) -> usize {
  seats.iter().map(|seat| seat.sid).max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn seat_from_str() {
    assert_eq!(Seat::from_str(&"FFFFFFFLLL"), Seat { row: 0, col: 0, sid: 0 });
    assert_eq!(Seat::from_str(&"FFFFFFBLLR"), Seat { row: 1, col: 1, sid: 9 });
    assert_eq!(Seat::from_str(&"BFFFFFFRLL"), Seat { row: 64, col: 4, sid: 516 });
    assert_eq!(Seat::from_str(&"BFFFFFBRLL"), Seat { row: 65, col: 4, sid: 524 });
    assert_eq!(Seat::from_str(&"BFBBBBBLLL"), Seat { row: 95, col: 0, sid: 760 });
  }
}
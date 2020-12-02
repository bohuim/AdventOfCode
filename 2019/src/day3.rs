use core::ops::RangeInclusive;
use std::cmp::{ min, max };

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point { x: i32, y: i32 }
impl Point {
  fn origin() -> Self { Point{x: 0, y: 0} }

  fn manhattan_dist_to(&self, other: Point) -> u32 {
    ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
  }

  fn manhattan_dist_origin(&self) -> u32 {
    self.manhattan_dist_to(Point::origin())
  }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Alignment { Ver, Hor, Other }

#[derive(Clone, Copy, Debug)]
pub struct Segment { 
  steps: u32,
  p1: Point, 
  p2: Point,
}
impl Segment {
  fn empty() -> Self {
    Segment{ steps: 0, p1: Point::origin(), p2: Point::origin() }
  }

  fn alignment(&self) -> Alignment {
    if self.p1 == self.p2 { Alignment::Other }
    else if self.p1.x == self.p2.x { Alignment::Ver }
    else if self.p1.y == self.p2.y { Alignment::Hor }
    else { Alignment::Other }
  }

  fn x_range(&self) -> RangeInclusive<i32> {
    min(self.p1.x, self.p2.x)..=max(self.p1.x, self.p2.x)
  }

  fn y_range(&self) -> RangeInclusive<i32> {
    min(self.p1.y, self.p2.y)..=max(self.p1.y, self.p2.y)
  }

  fn intersects(&self, other: Segment) -> Option<Point> {
    if (self.alignment() == Alignment::Other) || 
      (other.alignment() == Alignment::Other) ||
      (self.alignment() == other.alignment()) {
      return None;
    }

    let (x, y, x_range, y_range) =
      if self.alignment() == Alignment::Ver { 
        (self.p1.x, other.p1.y, other.x_range(), self.y_range())
      } else { 
        (other.p1.x, self.p1.y, self.x_range(), other.y_range())
      };

    if x_range.contains(&x) && y_range.contains(&y) { Some(Point {x: x, y: y}) }
    else { None }
  }
}

fn parse_path(input: &str) -> Vec<Segment> {
  let mut p1 = Point{x:0, y:0};
  let mut steps = 0;
  input
    .trim()
    .split(",")
    .map(|s| {
      let len = s[1..].parse::<i32>().unwrap();
      let seg = match &s[..1] {
        "U" => Segment{ steps: steps, p1: p1, p2: Point{ x: p1.x, y: p1.y + len } },
        "D" => Segment{ steps: steps, p1: p1, p2: Point{ x: p1.x, y: p1.y - len } },
        "L" => Segment{ steps: steps, p1: p1, p2: Point{ x: p1.x - len, y: p1.y } },
        "R" => Segment{ steps: steps, p1: p1, p2: Point{ x: p1.x + len, y: p1.y } },
        _ => Segment::empty()
      };
      steps += p1.manhattan_dist_to(seg.p2);
      p1 = seg.p2;
      seg
    })
    .collect()
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> [Vec<Segment>; 2] {
  let paths: Vec< Vec<Segment> > = input
    .lines()
    .map(|line| parse_path(line))
    .collect();
  [paths[0].clone(), paths[1].clone()]
}

#[aoc(day3, part1)]
pub fn part1(paths: &[Vec<Segment>; 2]) -> u32 {
  let path1 = paths[0].clone();
  let path2 = paths[1].clone();

  path1
    .iter()
    .map(|seg1| {
      path2
        .iter()
        .map(|seg2| seg1.intersects(*seg2))
        .filter_map(|pt| pt)
        .min_by_key(|pt| pt.manhattan_dist_origin())
    })
    .filter_map(|pt| pt)
    .min_by_key(|pt| pt.manhattan_dist_origin())
    .unwrap()
    .manhattan_dist_origin()
}


#[aoc(day3, part2)]
pub fn part2(paths: &[Vec<Segment>; 2]) -> u32 {
  let path1 = paths[0].clone();
  let path2 = paths[1].clone();

  path1
    .iter()
    .map(|seg1| {
      path2
        .iter()
        .map(|seg2| {
          match seg1.intersects(*seg2) {
            None => std::u32::MAX,
            Some(pt) => {
              let seg1_steps = seg1.steps + seg1.p1.manhattan_dist_to(pt);
              let seg2_steps = seg2.steps + seg2.p1.manhattan_dist_to(pt);
              let steps = seg1_steps + seg2_steps;
              steps
            }
          }
        })
        .min()
    })
    .filter_map(|steps| steps) // steps to intersection point
    .min()
    .unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hor_segment() {
    let s1 = Segment{ p1: Point{x:0, y:0}, p2: Point{x:10, y:0} };
    assert_eq!(s1.x_range(), 0..=10);
    assert_eq!(s1.y_range(), 0..=0);
    assert_eq!(s1.alignment(), Alignment::Hor);
  }

  #[test]
  fn test_ver_segment() {
    let s1 = Segment{ p1: Point{x:0, y:-5}, p2: Point{x:0, y:5} };
    assert_eq!(s1.x_range(), 0..=0);
    assert_eq!(s1.y_range(), -5..=5);
    assert_eq!(s1.alignment(), Alignment::Ver);
  }

  #[test]
  fn test_segment_intersects() {
    let s1 = Segment{ p1: Point{x:-2, y:0}, p2: Point{x:10, y:0} };
    let s2 = Segment{ p1: Point{x:1, y:-5}, p2: Point{x:1, y:5} };
    assert_eq!(s1.intersects(s2), Some(Point{x:1, y:0}));
  }
}

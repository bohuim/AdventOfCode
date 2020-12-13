use std::collections::{ HashMap, VecDeque };

const PREAMBLE_SIZE: usize = 25;

/// Represents a list of numbers that act as components, maintaining a both the order in which
/// numbers were added, and the occurences of each number.
#[derive(Debug)]
pub struct Preamble {
  order: VecDeque<usize>,
  count: HashMap<usize, usize>,
}
impl Preamble {
  /// Designated empty initializer.
  fn new() -> Preamble {
    Preamble { order: VecDeque::new(), count: HashMap::new() }
  }

  /// Designated initializer from an iterator of numbers.
  fn from<'a, I>(numbers: I) -> Preamble where I: Iterator<Item = &'a usize> {
    let mut preamble = Preamble::new();
    numbers.into_iter().for_each(|&x| preamble.insert(x));
    preamble
  }

  /// Inserts `new` increasing this preamble's length.
  fn insert(&mut self, new: usize) {
    self.order.push_back(new);
    *self.count.entry(new).or_insert(0) += 1;
  }

  /// Replaces the first number with `new`, retaining this preamble's length.
  fn replace(&mut self, new: usize) {
    let dec = self.order.pop_front().unwrap();
    assert!(self.count[&dec] > 0);
    self.count.entry(dec).and_modify(|c| *c -= 1);
    self.insert(new);
  }

  /// Checks whether `target` is a valid combination in this preamble.
  fn check(&self, target: usize) -> bool {
    self.count.iter().any(|(&x, &xc)| {
      let y = target - x;
      // If target is 2x, there must be 2+ instances of x.
      if x == y { xc > 1 } 
      // Otherwise there must be 1+ instances of y.
      else { self.count.get(&y).map_or(false, |&yc| yc > 0) }
    })
  }
}

/// Rolling window of numbers with a target sum.
pub struct Window {
  target: usize,
  sum: usize,
  values: VecDeque<usize>,
}
impl Window {
  /// Designated initializer from the target value.
  fn new(target: usize) -> Window {
    Window { target, sum: 0, values: VecDeque::new() }
  }

  /// Adds `new` to the back of the window, adjusting the window via `pop()` afterwards.
  fn add(&mut self, new: usize) -> Option<usize> {
    self.values.push_back(new);
    self.sum += new;
    self.pop()
  }

  /// Removes numbers from the front while sum is larger than the target. If sum is the target,
  /// adds the min and max of the window as the solution.
  fn pop(&mut self) -> Option<usize> {
    while self.sum > self.target {
      self.sum -= self.values.pop_front().unwrap();
    }
    if self.sum < self.target { None }
    else {
      let min = self.values.iter().min().unwrap();
      let max = self.values.iter().max().unwrap();
      Some(min + max)
    }
  }
}

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<usize> {
  input.trim().lines().filter_map(|x| x.trim().parse().ok()).collect()
}

#[aoc(day9, part1)]
pub fn solve1(numbers: &Vec<usize>) -> usize {
  let mut preamble = Preamble::from(numbers.iter().take(PREAMBLE_SIZE));
  for &x in &numbers[PREAMBLE_SIZE..] {
    if !preamble.check(x) {
      return x;
    }
    preamble.replace(x);
  }
  panic!("Part1 solution not found")
}

#[aoc(day9, part2)]
pub fn solve2(numbers: &Vec<usize>) -> usize {
  let mut window = Window::new(solve1(&numbers));
  for &x in numbers {
    if let Some(sol) = window.add(x) {
      return sol;
    }
  }
  panic!("Part2 solution not found")
}
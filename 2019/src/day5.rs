#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day5, part1)]
pub fn part1(n: &Vec<i32>) -> i32 {
  *run(&mut n.clone(), 1).last().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(n: &Vec<i32>) -> i32 {
  *run(&mut n.clone(), 5).last().unwrap()
}

pub fn run(n: &mut Vec<i32>, arg: i32) -> Vec<i32> {
  run_mult_args(n, vec![arg])
}

pub fn run_mult_args(n: &mut Vec<i32>, mut args: Vec<i32>) -> Vec<i32> {
  let len = n.len();
  let mut outputs = Vec::<i32>::new();

  let mut i = 0;
  while i < len && n[i] != 99 {
    let inst = n[i];
    let op = inst % 100;
    let m = inst / 100;

    i = 
      match op {
        1 => add(n, i, m),
        2 => mul(n, i, m),
        3 => put(n, i, args.remove(0)),
        4 => get(n, i, m, &mut outputs),
        5 => jit(n, i, m),
        6 => jif(n, i, m),
        7 => lts(n, i, m),
        8 => eqs(n, i, m),
        _ => len,
      }
  }
  outputs
}

fn add(n: &mut Vec<i32>, i: usize, m: i32) -> usize {
  let mode = parse_modes(m);
  let p1 = ret(n, n[i + 1], mode[0]);
  let p2 = ret(n, n[i + 2], mode[1]);
  let pr = n[i + 3] as usize; // Note: writes are only used as Mode::POS.
  n[pr] = p1 + p2;
  i + 4
}

fn mul(n: &mut Vec<i32>, i: usize, m: i32) -> usize {
  let mode = parse_modes(m);
  let p1 = ret(n, n[i + 1], mode[0]);
  let p2 = ret(n, n[i + 2], mode[1]);
  let pr = n[i + 3] as usize; // Note: writes are only used as Mode::POS.
  n[pr] = p1 * p2;
  i + 4
}

// Jump if true
fn jit(n: &mut Vec<i32>, i: usize, m: i32) -> usize {
  let mode = parse_modes(m);
  let cond = ret(n, n[i + 1], mode[0]);
  let line = ret(n, n[i + 2], mode[1]);
  if cond != 0 { line as usize } else { i + 3 }
}

// Jump if false
fn jif(n: &mut Vec<i32>, i: usize, m: i32) -> usize {
  let mode = parse_modes(m);
  let cond = ret(n, n[i + 1], mode[0]);
  let line = ret(n, n[i + 2], mode[1]);
  if cond == 0 { line as usize } else { i + 3 }
} 

// Less-than comparison
fn lts(n: &mut Vec<i32>, i: usize, m: i32) -> usize {
  let mode = parse_modes(m);
  let p1 = ret(n, n[i + 1], mode[0]);
  let p2 = ret(n, n[i + 2], mode[1]);
  let pr = n[i + 3] as usize; // Note: writes are only used as Mode::POS.
  n[pr] = if p1 < p2 { 1 } else { 0 };
  i + 4
}

// Equals comparison
fn eqs(n: &mut Vec<i32>, i: usize, m: i32) -> usize {
  let mode = parse_modes(m);
  let p1 = ret(n, n[i + 1], mode[0]);
  let p2 = ret(n, n[i + 2], mode[1]);
  let pr = n[i + 3] as usize; // Note: writes are only used as Mode::POS.
  n[pr] = if p1 == p2 { 1 } else { 0 };
  i + 4
}

// Puts n[p] = input
fn put(n: &mut Vec<i32>, i: usize, input: i32) -> usize {
  let pr = n[i + 1] as usize; // Note: writes are only used as Mode::POS.
  n[pr] = input;
  i + 2
}

// Gets and prints n[p]
fn get(n: &mut Vec<i32>, i: usize, m: i32, output: &mut Vec<i32>) -> usize {
  let mode = parse_modes(m);
  let val = ret(n, n[i + 1], mode[0]); 
  output.push(val);
  // println!("{}", val);
  i + 2
}

// Retreives value of `p` interpretted as position or immediate depending on mode `m`.
fn ret(n: &Vec<i32>, p: i32, m: Mode) -> i32 {
  match m {
    Mode::POS => n[p as usize],
    Mode::IMM => p,
  }
}

fn parse_modes(modes: i32) -> [Mode; 3] {
  let mut m = modes;
  let m1 = Mode::from(m % 10); m /= 10;
  let m2 = Mode::from(m % 10); m /= 10;
  let m3 = Mode::from(m % 10);
  [m1, m2, m3]
}

// DEFINITIONS --------------------

#[derive(Clone, Copy, Debug)]
enum Mode { POS, IMM }
impl Mode {
  fn from(x: i32) -> Self {
    match x {
      0 => Mode::POS,
      _ => Mode::IMM,
    }
  }
}

// TESTS --------------------

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test5p2_eq_pos() {
    // Output 1 if input is 8, otherwise.
    let n = vec![3,9,8,9,10,9,4,9,99,-1,8];
    assert_eq!(*run(&mut n.clone(), 8).last().unwrap(), 1);
    assert_eq!(*run(&mut n.clone(), 7).last().unwrap(), 0);
  }

  #[test]
  fn test5p2_lt_pos() {
    // Output 1 if input is less than 8, otherwise.
    let n = vec![3,9,7,9,10,9,4,9,99,-1,8];
    assert_eq!(*run(&mut n.clone(), 7).last().unwrap(), 1);
    assert_eq!(*run(&mut n.clone(), 8).last().unwrap(), 0);
    assert_eq!(*run(&mut n.clone(), 9).last().unwrap(), 0);
  }

  #[test]
  fn test5p2_eq_imm() {
    // Output 1 if input is 8, otherwise.
    let n = vec![3,3,1108,-1,8,3,4,3,99];
    assert_eq!(*run(&mut n.clone(), 8).last().unwrap(), 1);
    assert_eq!(*run(&mut n.clone(), 7).last().unwrap(), 0);
  }

  #[test]
  fn test5p2_lt_imm() {
    // Output 1 if input less than 8, otherwise.
    let n = vec![3,3,1107,-1,8,3,4,3,99];
    assert_eq!(*run(&mut n.clone(), 7).last().unwrap(), 1);
    assert_eq!(*run(&mut n.clone(), 8).last().unwrap(), 0);
    assert_eq!(*run(&mut n.clone(), 9).last().unwrap(), 0);
  }

  #[test]
  fn test5p2_jmp_pos() {
    // Output 0 if the input was zero or 1 if the input was non-zero.
    let n = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    assert_eq!(*run(&mut n.clone(), 5).last().unwrap(), 1);
    assert_eq!(*run(&mut n.clone(), 0).last().unwrap(), 0);
  }

  #[test]
  fn test5p2_jmp_imm() {
    // Output 0 if the input was zero or 1 if the input was non-zero
    let n = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
    assert_eq!(*run(&mut n.clone(), 5).last().unwrap(), 1);
    assert_eq!(*run(&mut n.clone(), 0).last().unwrap(), 0);
  }

  #[test]
  fn test5p2_long() {
    // Output:
    //  999 if input < 8
    // 1000 if input = 8
    // 1001 if input > 8
    let n = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
      1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
      999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    assert_eq!(*run(&mut n.clone(), 7).last().unwrap(),  999);
    assert_eq!(*run(&mut n.clone(), 8).last().unwrap(), 1000);
    assert_eq!(*run(&mut n.clone(), 9).last().unwrap(), 1001);
  }
}


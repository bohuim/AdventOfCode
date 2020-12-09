use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
  static ref RE_INST: Regex = Regex::new(r"(acc|jmp|nop) ((?:\-|\+)\d+)").unwrap();
}

/// Program is a list of instructions.
type Program = Vec<Inst>;

/// Represents the instruction set.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Inst {
  ACC(i64),
  JMP(i64),
  NOP(i64),
}
impl Inst {
  /// Parses an instruction from a line.
  fn from(line: &str) -> Inst {
    let groups = RE_INST.captures(line.trim()).unwrap();
    let arg: i64 = groups.get(2).unwrap().as_str().parse().unwrap();
    match groups.get(1).unwrap().as_str() {
      "acc" => Inst::ACC(arg),
      "jmp" => Inst::JMP(arg),
      "nop" => Inst::NOP(arg),
      _ => panic!("Unknown instruction opcode"),
    }
  }
}

/// Runs the given program until it completes or loops, returning a flag for whether it completed 
/// and the acc value at stop.
fn run(program: &Program) -> (bool, i64) {
  let last = program.len() as i64;
  let mut visited: HashSet<i64> = HashSet::new();
  let mut acc: i64 = 0;
  let mut i: i64 = 0;
  while !visited.contains(&i) {
    visited.insert(i);
    i = match &program[i as usize] {
      Inst::JMP(dff) => i + dff,
      Inst::ACC(add) => {
        acc += add;
        i + 1
      },
      Inst::NOP(_) => i + 1,
    };
    if i == last {
      return (true, acc);
    }
  }
  (false, acc)
}

/// Creates a modified program by swapping jmp <-> nop instructions at the given line `i`.
/// 
/// **Note**: Returns a copy of the program if not a jmp or nop instruction.
fn modified(original: &Program, i: usize) -> Program {
  let mut copied: Program = original.to_vec();
  copied[i] = match copied[i] {
    Inst::JMP(arg) => Inst::NOP(arg),
    Inst::NOP(arg) => Inst::JMP(arg),
    _ => copied[i].to_owned(),
  };
  copied
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Program {
  input.trim().lines().map(Inst::from).collect()
}

#[aoc(day8, part1)]
pub fn solve1(program: &Program) -> i64 {
  let (completed, acc) = run(&program);
  assert!(!completed, "Part1 program completed, expected loop");
  acc
}

#[aoc(day8, part2)]
pub fn solve2(program: &Program) -> i64 {
  (0..program.len())
    // filter for instruction lines that are jmp or nop
    .filter(|&i| match program[i] {
      Inst::JMP(_) | Inst::NOP(_) => true,
      _ => false,
    })
    // swap jmp <-> nop opcodes at the line and run
    .map(|i| run(&modified(&program, i)))
    // find the first modified program that completes
    .find(|&(completed, _)| completed)
    // get ending acc value of the modified program
    .unwrap().1
}

#[cfg(test)]
mod tests {
  use super::*;

  lazy_static! {
    static ref SAMPLE: Program = parse(r"
      nop +0
      acc +1
      jmp +4
      acc +3
      jmp -3
      acc -99
      acc +1
      jmp -4
      acc +6
    ");
  }

  #[test]
  fn inst_from() {
    assert_eq!(Inst::from("acc +42"), Inst::ACC(42));
    assert_eq!(Inst::from("jmp -13"), Inst::JMP(-13));
    assert_eq!(Inst::from("nop +0"), Inst::NOP(0));
    assert_eq!(Inst::from("nop -0"), Inst::NOP(0));
  }

  #[test]
  fn solve1_sample() {
    assert_eq!(solve1(&SAMPLE), 5);
  }

  #[test]
  fn solve2_sample() {
    assert_eq!(solve2(&SAMPLE), 8);
  }
}
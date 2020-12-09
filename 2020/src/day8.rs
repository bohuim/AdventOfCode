use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
  static ref RE_INST: Regex = Regex::new(r"(acc|jmp|nop) ((?:\-|\+)\d+)").unwrap();
}

/// Program is a list of instructions.
type Program = Vec<Inst>;

/// Represents the instruction set.
#[derive(PartialEq, Eq, Debug)]
pub enum Inst {
  ACC(i64),
  JMP(i64),
  NOP,
}
impl Inst {
  /// Parses an instruction from a line.
  fn from(line: &str) -> Option<Inst> {
    RE_INST.captures(line.trim())
      .map(|groups| match groups.get(1).unwrap().as_str() {
        "acc" => Inst::ACC(groups.get(2).unwrap().as_str().parse().unwrap()),
        "jmp" => Inst::JMP(groups.get(2).unwrap().as_str().parse().unwrap()),
        _ => Inst::NOP,
      })
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
      Inst::NOP => i + 1,
    };
    if i == last {
      return (true, acc);
    }
  }
  (false, acc)
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Program {
  input.trim().lines().filter_map(Inst::from).collect()
}

#[aoc(day8, part1)]
pub fn solve1(program: &Program) -> i64 {
  let (completed, acc) = run(&program);
  assert!(!completed, "Part1 program completed, expected loop");
  acc
}

#[cfg(test)]
mod tests {
  use super::*;

  lazy_static! {
    static ref SAMPLE1: Program = parse(r"
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
    assert_eq!(Inst::from("acc +42").unwrap(), Inst::ACC(42));
    assert_eq!(Inst::from("jmp -13").unwrap(), Inst::JMP(-13));
    assert_eq!(Inst::from("nop +0").unwrap(), Inst::NOP);
    assert_eq!(Inst::from("nop -0").unwrap(), Inst::NOP);
    assert_eq!(Inst::from("abc -0"), None);
  }

  #[test]
  fn solve1_sample1() {
    assert_eq!(solve1(&SAMPLE1), 5);
  }
}
extern crate superslice;
use superslice::*;
use crate::day5;

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<i32> {
  day5::parse(input)
}

#[aoc(day7, part1)]
pub fn part1(n: &Vec<i32>) -> i32 {
  let mut max_out = i32::min_value();

  let permutations = 5*4*3*2*1;
  let mut phase_seq = [0i32, 1, 2, 3, 4];
  for _ in 0..permutations {
    let out = amplify(n.clone(), &phase_seq);
    if out > max_out {
      max_out = out;
    }
    phase_seq.next_permutation();
  }
  max_out
}

fn amplify(mut program: Vec<i32>, phase_seq: &[i32; 5]) -> i32 {
  let mut amps = [0i32; 5];
  for i in 0..5 {
    let input = if i == 0 { 0 } else { amps[i-1] };
    let output = day5::run_mult_args(&mut program, vec![phase_seq[i], input]);
    amps[i] =
      match output.len() {
        0 => i32::min_value(),
        l => output[l - 1],
      }
  }
  amps[4]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test7p1_t1() {
    let program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    assert_eq!(part1(&program), 43210);
  }

  #[test]
  fn test7p1_t2() {
    let program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                       101,5,23,23,1,24,23,23,4,23,99,0,0];
    assert_eq!(part1(&program), 54321);
  }

  #[test]
  fn test7p1_t3() {
    let program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                       1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    assert_eq!(part1(&program), 65210);
  }
}


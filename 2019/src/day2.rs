#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<i32> {
  input
    .split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day2, part1)]
pub fn part1(n: &Vec<i32>) -> i32 {
  run(n, 12, 2)  
}

#[aoc(day2, part2)]
pub fn part2(n: &Vec<i32>) -> i32 {
  for noun in 0..100 {
    for verb in 0..100 {
      if run(&n, noun, verb) == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  return -1
}

pub fn run(program: &Vec<i32>, noun: i32, verb: i32) -> i32 {
  let mut n = program.clone();
  n[1] = noun;
  n[2] = verb;

  let mut i = 0;
  while n[i] != 99 {
    let op = n[i];
    let pos1 = n[i+1] as usize;
    let pos2 = n[i+2] as usize;
    let posr = n[i+3] as usize;
    
    let x1 = n[pos1];
    let x2 = n[pos2];

    let r = if op == 1 { x1 + x2 } else { x1 * x2 };
    n[posr] = r;

    i += 4;
  }

  n[0]
}

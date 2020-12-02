use regex::Regex;

#[aoc(day2, part1)]
pub fn solve1(input: &str) -> usize {
  let matcher = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z0-9]+)$").unwrap();
  input.lines()
    .filter(|line| {
      let groups = matcher.captures(line).unwrap();
      let beg: usize = groups.get(1).unwrap().as_str().parse().unwrap();
      let end: usize = groups.get(2).unwrap().as_str().parse().unwrap();
      let ch = groups.get(3).unwrap().as_str();
      let pw = groups.get(4).unwrap().as_str();
      let count = pw.matches(&ch).count();
      let valid = (beg..end+1).contains(&count);
      valid 
    })
    .count()
}
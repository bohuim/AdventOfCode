#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|l| l.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
pub fn part1(masses: &Vec<i32>) -> i32 {
  masses
    .iter()
    .map(fuel_for_mass)
    .sum()
}

#[aoc(day1, part2)]
pub fn part2(masses: &Vec<i32>) -> i32 {
  masses
    .iter()
    .map(|m| total_fuel(*m, 0))
    .sum()
}

fn fuel_for_mass(mass: &i32) -> i32 {
  mass / 3 - 2
}

fn total_fuel(mass: i32, sum: i32) -> i32 {
  let fuel = fuel_for_mass(&mass);
  if fuel <= 0 { return sum; }

  total_fuel(fuel, sum + fuel)
}

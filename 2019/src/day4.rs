use core::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn parse(input: &str) -> RangeInclusive<u32> {
  let nums: Vec<u32> = input
    .split("-")
    .map(|numstr| numstr.parse::<u32>().unwrap())
    .collect();
  RangeInclusive::new(nums[0], nums[1])
}

#[aoc(day4, part1)]
pub fn part1(range: &RangeInclusive<u32>) -> usize {
  range.clone()
    .into_iter()
    .filter(is_valid_password)
    .count()
}

fn is_valid_password(n: &u32) -> bool {
  // Assumed to be 6 digits.
  let mut left = n.to_owned();
  let mut digits: [u32; 6] = [0; 6];
  for i in 0..6 {
    digits[5 - i] = left % 10;
    left /= 10;
  }

  let mut ascending = true;
  let mut consecutive = false;
  for i in 0..5 {
    let d1 = digits[i];
    let d2 = digits[i + 1];
    ascending &= d1 <= d2;
    consecutive |= d1 == d2;
  }

  ascending && consecutive
}


#[aoc(day4, part2)]
pub fn part2(range: &RangeInclusive<u32>) -> usize {
  range.clone()
    .into_iter()
    .filter(is_valid_password)
    .filter(is_valid_password2)
    .count()
}

fn is_valid_password2(n: &u32) -> bool {
  // Assumed to be 6 digits.
  let mut left = n.to_owned();
  let mut digits: [u32; 6] = [0; 6];
  for i in 0..6 {
    digits[5 - i] = left % 10;
    left /= 10;
  }

  let mut ascending = true;
  let mut consecutive = false;
  let mut repeats = [0u32; 10];
  for i in 0..5 {
    let d1 = digits[i];
    let d2 = digits[i + 1];
    ascending &= d1 <= d2;
    if d1 == d2 {
      consecutive = true;
      repeats[d1 as usize] += 1;
    }
  }

  ascending && 
    consecutive &&
    *repeats.iter().filter(|c| **c > 0).min().unwrap() == 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test04_is_valid_password2() {
    assert!( is_valid_password2(&111144));
    assert!(!is_valid_password2(&123444));
    assert!(!is_valid_password2(&122224));
    assert!( is_valid_password2(&122255));
    assert!(!is_valid_password2(&122222));
  }
}

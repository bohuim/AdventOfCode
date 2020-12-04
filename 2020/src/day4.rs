use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

// All field types as static slices.
static BYR: &str = "byr";
static IYR: &str = "iyr";
static EYR: &str = "eyr";
static HGT: &str = "hgt";
static HCL: &str = "hcl";
static ECL: &str = "ecl";
static PID: &str = "pid";

// Regex matchers for fields.
lazy_static! {
  static ref RE_HGT: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
  static ref RE_HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
  static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
  static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

/// Represents a passport entry as a hash map of `String -> String`.
pub struct Passport {
  field: HashMap<String, String>
}
impl Passport {
  /// Initializer from string of whitespace separated "field:value" tokens.
  fn from_str(input: &str) -> Passport {
    Passport {
      field: 
        input.split_whitespace()
          .map(|entry| { // "field:value" token
            let v: Vec<&str> = entry.split(":").collect();
            (v[0].to_string(), v[1].to_string())
          })
          .collect()
    }
  }

  /// Checks whether self has all fields except `Field::CountryID`.
  fn validate1(&self) -> bool {
       self.field.get(BYR).is_some()
    && self.field.get(IYR).is_some()
    && self.field.get(EYR).is_some()
    && self.field.get(HGT).is_some()
    && self.field.get(HCL).is_some()
    && self.field.get(ECL).is_some()
    && self.field.get(PID).is_some()
  }

  /// Check whether self has valid values in all fields except `Field::CountryID`.
  fn validate2(&self) -> bool {
       self.check_byr()
    && self.check_iyr()
    && self.check_eyr()
    && self.check_hgt()
    && self.check_hcl()
    && self.check_ecl()
    && self.check_pid()
  }

  /// Checks whether the BYR (birth year) value is correct.
  fn check_byr(&self) -> bool {
    (1920..=2002).contains(&self.field.get(BYR).map_or(0, |x| x.parse::<u16>().unwrap_or(0)))
  }

  /// Checks whether the IYR (issue year) value is correct.
  fn check_iyr(&self) -> bool {
    (2010..=2020).contains(&self.field.get(IYR).map_or(0, |x| x.parse::<u16>().unwrap_or(0)))
  }

  /// Checks whether the EYR (expiration year) value is correct.
  fn check_eyr(&self) -> bool {
    (2020..=2030).contains(&self.field.get(EYR).map_or(0, |x| x.parse::<u16>().unwrap_or(0)))
  }

  /// Checks whether the HGT (height) value is correct.
  fn check_hgt(&self) -> bool {
    RE_HGT
      // capture groups using HGT field as slice.
      .captures(self.field.get(HGT).map_or("", String::as_str))
      // if no value or invalid format, return false early.
      .map_or(false, |groups| {
        if groups.get(2).is_none() { return false; }
        // verify range depending on the unit
        let length: u16 = groups[1].parse().unwrap_or(0);
        match &groups[2] {
          "cm" => (150..=193).contains(&length),
          "in" => (59..=76).contains(&length),
          _ => false,
        }
      })
  }

  /// Checks whether the HCL (hair color) value is correct.
  fn check_hcl(&self) -> bool {
    RE_HCL.is_match(self.field.get(HCL).map_or("", String::as_str))
  }

  /// Checks whether the ECL (eye color) value is correct.
  fn check_ecl(&self) -> bool {
    RE_ECL.is_match(self.field.get(ECL).map_or("", String::as_str))
  }

  /// Checks whether the PID (passport ID) value is correct.
  fn check_pid(&self) -> bool {
    RE_PID.is_match(self.field.get(PID).map_or("", String::as_str))
  }
}

//#mark - Solution

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Passport> {
  input.split("\n\n").map(Passport::from_str).collect()
}

//#mark - Tests

#[aoc(day4, part1)]
pub fn solve1(passports: &Vec<Passport>) -> usize {
  passports.iter().filter(|p| p.validate1()).count()
}

#[aoc(day4, part2)]
pub fn solve2(passports: &Vec<Passport>) -> usize {
  passports.iter().filter(|p| p.validate2()).count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_validate1() {
    assert!(Passport::from_str(&"byr:a iyr:a eyr:a hgt:a hcl:a ecl:a pid:a cid:a").validate1());
    assert!(Passport::from_str(&"byr:a iyr:a eyr:a hgt:a hcl:a ecl:a pid:a").validate1());
    assert!(!Passport::from_str(&"byr:a eyr:a hgt:a hcl:a ecl:a pid:a cid:a").validate1());
    assert!(!Passport::from_str(&"byr:a iyr:a hgt:a hcl:a ecl:a pid:a").validate1());
    assert!(!Passport::from_str(&"hgt:a pid:a ecl:a byr:a eyr:a iyr:a cid:a").validate1());
  }

  #[test]
  fn test_validate2() {
    assert!(Passport::from_str(&"byr:a iyr:a eyr:a hgt:a hcl:a ecl:a pid:a cid:a").validate1());
    assert!(Passport::from_str(&"byr:a iyr:a eyr:a hgt:a hcl:a ecl:a pid:a").validate1());
    assert!(!Passport::from_str(&"byr:a eyr:a hgt:a hcl:a ecl:a pid:a cid:a").validate1());
    assert!(!Passport::from_str(&"byr:a iyr:a hgt:a hcl:a ecl:a pid:a").validate1());
    assert!(!Passport::from_str(&"hgt:a pid:a ecl:a byr:a eyr:a iyr:a cid:a").validate1());
  }

  #[test]
  fn test_check_byr() {
    assert!(!Passport::from_str(&"byr:1919").check_byr());
    assert!( Passport::from_str(&"byr:1920").check_byr());
    assert!( Passport::from_str(&"byr:1996").check_byr());
    assert!( Passport::from_str(&"byr:2002").check_byr());
    assert!(!Passport::from_str(&"byr:2003").check_byr());
  }

  #[test]
  fn test_check_iyr() {
    assert!(!Passport::from_str(&"iyr:2009").check_iyr());
    assert!( Passport::from_str(&"iyr:2010").check_iyr());
    assert!( Passport::from_str(&"iyr:2018").check_iyr());
    assert!( Passport::from_str(&"iyr:2020").check_iyr());
    assert!(!Passport::from_str(&"iyr:2021").check_iyr());
  }

  #[test]
  fn test_check_eyr() {
    assert!(!Passport::from_str(&"eyr:2019").check_eyr());
    assert!( Passport::from_str(&"eyr:2020").check_eyr());
    assert!( Passport::from_str(&"eyr:2025").check_eyr());
    assert!( Passport::from_str(&"eyr:2030").check_eyr());
    assert!(!Passport::from_str(&"eyr:2031").check_eyr());
  }

  #[test]
  fn test_check_hgt() {
    assert!(!Passport::from_str(&"hgt:a").check_hgt());
    assert!(!Passport::from_str(&"hgt:69").check_hgt());
    assert!(!Passport::from_str(&"hgt:135").check_hgt());
    assert!(!Passport::from_str(&"hgt:140cm").check_hgt());
    assert!(!Passport::from_str(&"hgt:200cm").check_hgt());
    assert!(!Passport::from_str(&"hgt:40in").check_hgt());
    assert!(!Passport::from_str(&"hgt:80in").check_hgt());
    assert!( Passport::from_str(&"hgt:150cm").check_hgt());
    assert!( Passport::from_str(&"hgt:193cm").check_hgt());
    assert!( Passport::from_str(&"hgt:59in").check_hgt());
    assert!( Passport::from_str(&"hgt:76in").check_hgt());
  }

  #[test]
  fn test_check_hcl() {
    assert!(!Passport::from_str(&"hcl:a#").check_hcl());
    assert!(!Passport::from_str(&"hcl:123").check_hcl());
    assert!(!Passport::from_str(&"hcl:#123").check_hcl());
    assert!( Passport::from_str(&"hcl:#ffffff").check_hcl());
    assert!( Passport::from_str(&"hcl:#d2d8d3").check_hcl());
  }

  #[test]
  fn test_check_ecl() {
    assert!(!Passport::from_str(&"ecl:a").check_ecl());
    assert!(!Passport::from_str(&"ecl:123").check_ecl());
    assert!(!Passport::from_str(&"ecl:abc").check_ecl());
    assert!( Passport::from_str(&"ecl:amb").check_ecl());
    assert!( Passport::from_str(&"ecl:blu").check_ecl());
    assert!( Passport::from_str(&"ecl:brn").check_ecl());
    assert!( Passport::from_str(&"ecl:gry").check_ecl());
    assert!( Passport::from_str(&"ecl:grn").check_ecl());
    assert!( Passport::from_str(&"ecl:hzl").check_ecl());
    assert!( Passport::from_str(&"ecl:oth").check_ecl());
  }

  #[test]
  fn test_check_pid() {
    assert!(!Passport::from_str(&"pid:a").check_pid());
    assert!(!Passport::from_str(&"pid:123").check_pid());
    assert!(!Passport::from_str(&"pid:1234567890").check_pid());
    assert!( Passport::from_str(&"pid:123456789").check_pid());
    assert!( Passport::from_str(&"pid:000000000").check_pid());
    assert!( Passport::from_str(&"pid:000000001").check_pid());
    assert!( Passport::from_str(&"pid:100000000").check_pid());
  }
}
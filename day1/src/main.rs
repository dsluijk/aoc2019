use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
  let input = File::open("./input.txt").expect("Input file not found.");
  let reader = BufReader::new(input);
  let mut result: u64 = 0;

  for (_, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let weight: u64 = line.parse().expect("Line is not a number!");
    result += calculate_fuel(weight);
  }

  println!("Fuel requirement: {}", result);
}

fn calculate_fuel(weight: u64) -> u64 {
  let mut fuel_weight = weight / 3;

  if fuel_weight <= 2 {
    return 0;
  }

  fuel_weight -= 2;

  return fuel_weight + calculate_fuel(fuel_weight);
}

#[cfg(test)]
mod tests {
  use crate::calculate_fuel;

  #[test]
  fn fuel_small() {
    assert_eq!(calculate_fuel(14), 2);
  }

  #[test]
  fn fuel_not_single() {
    assert_ne!(calculate_fuel(1969), 654);
  }

  #[test]
  fn fuel_light() {
    assert_eq!(calculate_fuel(1969), 966);
  }

  #[test]
  fn fuel_heavy() {
    assert_eq!(calculate_fuel(100756), 50346);
  }
}
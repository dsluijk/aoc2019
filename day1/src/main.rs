fn main() {
  let input = String::from(include_str!("./input.txt"));
  let mut result: i64 = 0;

  for (_, line) in input.lines().enumerate() {
    let weight: i64 = line.parse().expect("Line is not a number!");
    result += calculate_fuel(weight);
  }

  println!("Fuel requirement: {}", result);
}

fn calculate_fuel(weight: i64) -> i64 {
  let fuel_weight = (weight / 3) - 2;

  if fuel_weight <= 0 {
    return 0;
  }

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
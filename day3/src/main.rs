fn main() {
    let input = String::from(include_str!("input.txt"));
    let mut split = input.lines();
    let path1 = String::from(split.next().expect("Not enough input lines"));
    let path2 = String::from(split.next().expect("Not enough input lines"));

    println!("Distance: {}", distance(path1, path2));
}

fn distance(path1: String, path2: String) -> i32 {
    let vec1 = get_path(path1);
    let vec2 = get_path(path2);

    let mut intersections: Vec<(i32, i32)> = Vec::new();
    for one in vec1 {
        for two in &vec2 {
            if one.0 == two.0 && one.1 == two.1 {
                intersections.push(one);
            }
        }
    }

    let mut smallest: i32 = std::i32::MAX;

    for intersection in intersections {
        if intersection.0 == 0 && intersection.1 == 0 {
            continue;
        }

        let distance = i32::abs(intersection.0) + i32::abs(intersection.1);

        if distance < smallest {
            smallest = distance;
        }
    }

    return smallest;
}

fn get_path(path: String) -> Vec<(i32, i32)> {
    let mut vec: Vec<(i32, i32)> = Vec::new();
    let mut location = (0, 0);

    for splitted in path.split(",") {
        let movement = String::from(splitted);
        let mut chars = movement.chars();
        let direction = chars.next().unwrap();
        let amount = chars.as_str().parse::<i32>().expect("Could not parse number!");

        for _ in 0..amount {
            match direction {
                'R' => {
                    let new_pair: (i32, i32) = (location.0 + 1, location.1);
                    vec.push(new_pair);
                    location = new_pair;
                },
                'L' => {
                    let new_pair: (i32, i32) = (location.0 - 1, location.1);
                    vec.push(new_pair);
                    location = new_pair;
                },
                'U' => {
                    let new_pair: (i32, i32) = (location.0, location.1 + 1);
                    vec.push(new_pair);
                    location = new_pair;
                },
                'D' => {
                    let new_pair: (i32, i32) = (location.0, location.1 - 1);
                    vec.push(new_pair);
                    location = new_pair;
                },
                _ => panic!("Invalid movement!"),
            }
        }
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use crate::distance;

    #[test]
    fn example_1() {
        let path1 = String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = String::from("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(distance(path1, path2), 159);
    }

    #[test]
    fn example_2() {
        let path1 = String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_ne!(distance(path1, path2), 135);
    }
}
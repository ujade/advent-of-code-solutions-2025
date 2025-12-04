advent_of_code::solution!(3);


fn find_joltage_p1(s: &str) -> u64 {
    let digits: Vec<u32> = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    (0..digits.len())
        .flat_map(|i| (i+1..digits.len()).map(move |j| (i, j)))
        .map(|(i, j)| digits[i] * 10 + digits[j])
        .max()
        .unwrap_or(0) as u64
}

fn find_joltage_p2(s: &str) -> u64 {
    let digits: Vec<u32> = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    let mut result = Vec::new();
    let mut start = 0;
    
    for pos in 0..12 {
        let remaining = 12 - pos - 1;
        let end = digits.len() - remaining;
        
        let max_val = (start..end).map(|i| digits[i]).max().unwrap();
        let max_idx = (start..end)
            .find(|&i| digits[i] == max_val)
            .unwrap();
        
        result.push(digits[max_idx]);
        start = max_idx + 1;
    }
    
    result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split("\n").collect::<Vec<&str>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let battery_arrays = parse_input(input);

    let mut added_joltage = 0u64;

    for battery in battery_arrays {

        let joltage = find_joltage_p1(battery);

        println!("| Joltage: {} | Battery: {}", joltage, battery);

        added_joltage += joltage;
    }

    Some(added_joltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_arrays = parse_input(input);

    let mut added_joltage = 0u64;

    for battery in battery_arrays {

        let joltage = find_joltage_p2(battery);

        println!("| Joltage: {} | Battery: {}", joltage, battery);

        added_joltage += joltage;
    }

    Some(added_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

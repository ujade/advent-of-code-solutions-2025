advent_of_code::solution!(2);


pub fn parse_id_ranges(input: &str) -> Option<Vec<(u64, u64)>> {
    input
        .trim()
        .split(',')
        .map(|product_id_range| {
            let product_id_range = product_id_range.trim();
            let mut parts = product_id_range.split('-');
            let start = parts.next()?.parse::<u64>().ok()?;
            let end = parts.next()?.parse::<u64>().ok()?;
            
            if parts.next().is_some() || start > end {
                return None;
            }
            Some((start, end))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    
    let ranges = parse_id_ranges(input)?;

    let mut total = 0;
    
    for (start, end) in ranges {
        for id in start..=end {
            let num_of_digits = (id as f64).log10().floor() as u64 + 1;

            if num_of_digits % 2 != 0 {
                        // skip odd-length numbers
                continue;
            }
    
            let divisor = 10_u64.pow((num_of_digits / 2).try_into().unwrap());
            // create a power of 10 that sits at midpoint e.g. divisor = 10^(4 / 2) = 10² = 100, then 1010 / 100 = 10.1
            let left = id / divisor;
            // extract left half of number
            let right = id % divisor;
            // extract right half of number

            if right == left {
                total+=id;
            }
        }
    }

    println!("total: {}", total);
    
    Some(total)
}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    (1..len).any(|seq_len| {
        // try each sequence length that divides evenly
        if len % seq_len != 0 {
            return false;
        }
        
        let pattern = &s[..seq_len];
        s.as_bytes()
            .chunks(seq_len)
            .all(|chunk| chunk == pattern.as_bytes())
    })
}


pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_id_ranges(input)?;

    let mut total_id = 0u64;

        for (start, end) in ranges {
            for id in start..=end {
                let id_length = id.to_string().len();

                // // let num_of_repeats = 0;
                // let id_str = id.to_string();

                //     let test = (2..=id_length/2).any(|skip| {
                //         id_str.chars()
                //             .zip(id_str.chars().skip(skip))
                //             .any(|(a, b)| a == b)
                //     });

                    if is_invalid_id(id) == true {
                        total_id += id;
                    }

            }
    }

    Some(total_id)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse_id_ranges(input).unwrap();
        
        println!("Value: {:?}", result);

        assert_eq!(result.len(), 11);
        assert_eq!(result[0], (11, 22));
        assert_eq!(result[1], (95, 115));
        assert_eq!(result[10], (2121212118, 2121212124));
    }


    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

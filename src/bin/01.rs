use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let num_iter = line.chars().filter(|c| c.is_ascii_digit());
        let first_num = num_iter.clone().next().unwrap();
        let second_num = num_iter.last().unwrap();

        let number = String::from(first_num) + &String::from(second_num);
        sum += number.parse::<u32>().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let string_to_digit: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut sum: u32 = 0;
    for line in input.lines() {
        let char_array = line.chars().collect::<Vec<char>>();

        let mut first_char = '!';
        let mut second_char = '!';

        let mut slice_start = 0;
        let mut slice_end = 0;

        while slice_start < char_array.len() {
            if slice_start == slice_end {
                if char_array[slice_start].is_ascii_digit() {
                    if first_char == '!' {
                        first_char = char_array[slice_start];
                    }
                    second_char = char_array[slice_start];
                }
            }

            if slice_end - slice_start > 5 {
                slice_start += 1;
                slice_end = slice_start;
                continue;
            }

            let pot_digit = string_to_digit.get(
                    &*char_array[slice_start..slice_end]
                        .iter()
                        .collect::<String>(),
                );
                if let Some(num) = pot_digit {
                    if first_char == '!' {
                        first_char = *num;
                    }
                    second_char = *num;
                }

            slice_end += 1;
            if slice_end > char_array.len() {
                slice_start += 1;
                slice_end = slice_start;
            }
        }
        let number = String::from(first_char) + &String::from(second_char);
        sum += number.parse::<u32>().unwrap();
        // println!("Total {}", number.parse::<u32>().unwrap())
    }
    Some(sum)
}

advent_of_code::main!("01");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "01"));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "01-2"));
        assert_eq!(result, Some(281));
    }
}

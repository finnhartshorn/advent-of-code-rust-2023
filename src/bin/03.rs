use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut full_array: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        full_array.push(line.chars().collect::<Vec<char>>());
    }

    for i in 0..full_array.len() {
        let mut current_number = String::from("");

        let mut slice_start = 0;
        let mut slice_end = 0;

        while slice_start < full_array[i].len() {
            if slice_end < full_array[i].len() && full_array[i][slice_end].is_ascii_digit() {
                current_number.push(full_array[i][slice_end]);
                slice_end += 1;
            } else {
                if !current_number.is_empty() {
                    let mut adjacent = false;
                    for x in slice_start..slice_end {
                        adjacent = adjacent || check_adjacent(&full_array, i, x)
                    }
                    if adjacent {
                        sum += current_number.parse::<u32>().unwrap()
                    }
                }
                slice_end += 1;
                slice_start = slice_end;
                current_number = String::from("");
            }
        }
    }

    Some(sum)
}

fn check_adjacent(array: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let not_symbols = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    if x > 0 && !not_symbols.contains(&array[x - 1][y]) {
        return true;
    }
    if x < array.len() - 1 && !not_symbols.contains(&array[x + 1][y]) {
        return true;
    }
    if y > 0 && !not_symbols.contains(&array[x][y - 1]) {
        return true;
    }
    if y < array[x].len() - 1 && !not_symbols.contains(&array[x][y + 1]) {
        return true;
    }
    if x > 0 && y > 0 && !not_symbols.contains(&array[x - 1][y - 1]) {
        return true;
    }
    if x > 0 && y < array[x].len() - 1 && !not_symbols.contains(&array[x - 1][y + 1]) {
        return true;
    }
    if x < array.len() - 1 && y > 0 && !not_symbols.contains(&array[x + 1][y - 1]) {
        return true;
    }
    if x < array.len() - 1 && y < array[x].len() - 1 && !not_symbols.contains(&array[x + 1][y + 1])
    {
        return true;
    }
    false
}

fn find_adjacent_asterix_coordinate(array: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<String> {
    let mut adjacent_asterix_coordinates: Vec<String> = Vec::new();
    if x > 0 && array[x - 1][y] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x - 1, y));
    }
    if x < array.len() - 1 && array[x + 1][y] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x + 1, y));
    }
    if y > 0 && array[x][y - 1] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x, y - 1));
    }
    if y < array[x].len() - 1 && array[x][y + 1] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x, y + 1));
    }
    if x > 0 && y > 0 && array[x - 1][y - 1] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x - 1, y - 1));
    }
    if x > 0 && y < array[x].len() - 1 && array[x - 1][y + 1] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x - 1, y + 1));
    }
    if x < array.len() - 1 && y > 0 && array[x + 1][y - 1] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x + 1, y - 1));
    }
    if x < array.len() - 1 && y < array[x].len() - 1 && array[x + 1][y + 1] == '*' {
        adjacent_asterix_coordinates.push(format!("{},{}", x + 1, y + 1));
    }
    adjacent_asterix_coordinates
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut full_array: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        full_array.push(line.chars().collect::<Vec<char>>());
    }

    let mut asterix_map: HashMap<String, Vec<String>> = HashMap::new();

    for i in 0..full_array.len() {
        let mut current_number = String::from("");

        let mut slice_start = 0;
        let mut slice_end = 0;

        while slice_start < full_array[i].len() {
            if slice_end < full_array[i].len() && full_array[i][slice_end].is_ascii_digit() {
                current_number.push(full_array[i][slice_end]);
                slice_end += 1;
            } else {
                if !current_number.is_empty() {
                    let mut adjacent_asterixes = HashSet::new();
                    for x in slice_start..slice_end {
                        find_adjacent_asterix_coordinate(&full_array, i, x)
                            .iter()
                            .for_each(|x| {
                                adjacent_asterixes.insert(x.clone());
                            });
                    }
                    adjacent_asterixes.iter().for_each(|x| {
                        if !asterix_map.contains_key(x) {
                            asterix_map.insert(x.clone(), Vec::new());
                        }
                        asterix_map.get_mut(x).unwrap().push(current_number.clone());
                    });
                }
                slice_end += 1;
                slice_start = slice_end;
                current_number = String::from("");
            }
        }
    }

    let mut sum = 0;
    for part_list in asterix_map.values() {
        if part_list.len() == 2 {
            sum += part_list[0].parse::<u32>().unwrap() * part_list[1].parse::<u32>().unwrap();
        }
    }

    Some(sum)
}

advent_of_code::main!("03");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "03"));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "03"));
        assert_eq!(result, Some(467835));
    }
}

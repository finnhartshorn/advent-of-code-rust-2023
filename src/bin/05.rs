use std::collections::HashMap;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.split("\n\n").map(|x| x.split(":").nth(1).unwrap());
    let seeds = iter.next().unwrap();
    seeds.split_whitespace().map(|x| x.parse::<u32>().unwrap()).map(|s| {
        let mut current_number = s;
        let mut next_number = s;
        iter.clone().for_each(|maps| {
            maps.trim().lines().for_each(|mapper| {
                let mut inner_itter = mapper.split_whitespace().map(|x| x.parse::<u32>().unwrap());
                let start_dest= inner_itter.next().unwrap();
                let start_source = inner_itter.next().unwrap();
                let range = inner_itter.next().unwrap();
                if let Some(n) = map_to_dest(start_source, start_dest, range, current_number) {
                    next_number = n;
                }
            });
            current_number = next_number;
        });
        next_number
    }).min()
}

fn map_to_dest(start_source: u32, start_dest: u32, range: u32, value: u32) -> Option<u32> {
    if value >= start_source && value < start_source + range {
        Some(start_dest + value - start_source)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.split("\n\n").map(|x| x.split(":").nth(1).unwrap());
    let seeds = iter.next().unwrap();
    let mut full_seeds: Vec<u32> = vec![];
    for mut chunk in &seeds.split_whitespace().chunks(2) {
        let start = chunk.next().unwrap().parse::<u32>().unwrap();
        let range = chunk.next().unwrap().parse::<u32>().unwrap();
        let vec: Vec<u32> = (start..start+range).collect();
        full_seeds.extend(vec);
    }

    let preprocessed_maps: Vec<Vec<u32>> = iter.map(|mapper| {
        mapper.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    }).collect();

    full_seeds.into_iter().map(|s| {
        let mut current_number = s;
        let mut next_number = s;
        preprocessed_maps.clone().into_iter().for_each(|maps| {
            for chunk in maps.chunks(3) {
                let start_dest = chunk[0];
                let start_source = chunk[1];
                let range = chunk[2];
                if let Some(n) = map_to_dest(start_source, start_dest, range, current_number) {
                    next_number = n;
                }
            }
            current_number = next_number;
        });
        next_number
    }).min()
}

pub fn part_two_backup(input: &str) -> Option<u32> {
    let mut iter = input.split("\n\n").map(|x| x.split(":").nth(1).unwrap());
    let seeds = iter.next().unwrap();
    let mut full_seeds: Vec<u32> = vec![];
    for mut chunk in &seeds.split_whitespace().chunks(2) {
        let start = chunk.next().unwrap().parse::<u32>().unwrap();
        let range = chunk.next().unwrap().parse::<u32>().unwrap();
        let vec: Vec<u32> = (start..start+range).collect();
        full_seeds.extend(vec);
    }
    full_seeds.into_iter().map(|s| {
        let mut current_number = s;
        let mut next_number = s;
        iter.clone().for_each(|maps| {
            maps.trim().lines().for_each(|mapper| {
                let mut inner_itter = mapper.split_whitespace().map(|x| x.parse::<u32>().unwrap());
                let start_dest= inner_itter.next().unwrap();
                let start_source = inner_itter.next().unwrap();
                let range = inner_itter.next().unwrap();
                if let Some(n) = map_to_dest(start_source, start_dest, range, current_number) {
                    next_number = n;
                }
            });
            current_number = next_number;
        });
        next_number
    }).min()
}

// pub fn part_two(input: &str) -> Option<u32> {
//     let mut iter = input.split("\n\n").map(|x| x.split(":").nth(1).unwrap());
//     let seeds = iter.next().unwrap();
//     let mut full_seeds: Vec<u32> = vec![];
//     for mut chunk in &seeds.split_whitespace().chunks(2) {
//         let start = chunk.next().unwrap().parse::<u32>().unwrap();
//         let range = chunk.next().unwrap().parse::<u32>().unwrap();
//         let vec: Vec<u32> = (start..start+range).collect();
//         full_seeds.extend(vec);
//     }
//
//     let mut preprocessed_maps: Vec<Vec<u32>> = iter.map(|mapper| {
//         mapper.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()
//     }).collect();
//
//     let end_map = preprocessed_maps.pop().unwrap();
//     let dest_to_source = HashMap::new();
//     let mut starting_numbers = end_map.clone().chunks(3).for_each(|x| x[0]).collect::<Vec<u32>>();
//     starting_numbers.sort();
//
//
//
//
//     None
// }

advent_of_code::main!("05");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "05"));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "05"));
        assert_eq!(result, Some(46));
    }
}

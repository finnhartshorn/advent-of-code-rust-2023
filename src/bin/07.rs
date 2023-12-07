use std::collections::HashMap;

use phf::phf_map;

static CARD_HEX: phf::Map<char, char> = phf_map! {
    'A' => 'D',
    'K' => 'C',
    'Q' => 'B',
    'J' => 'A',
    'T' => '9',
    '9' => '8',
    '8' => '7',
    '7' => '6',
    '6' => '5',
    '5' => '4',
    '4' => '3',
    '3' => '2',
    '2' => '1',
};

static CARD_HEX_2: phf::Map<char, char> = phf_map! {
    'A' => 'D',
    'K' => 'C',
    'Q' => 'B',
    'T' => 'A',
    '9' => '9',
    '8' => '8',
    '7' => '7',
    '6' => '6',
    '5' => '5',
    '4' => '4',
    '3' => '3',
    '2' => '2',
    'J' => '1',
};

struct Hand<'a> {
    cards: &'a str,
    bet: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            Hand {
                cards: iter.next().unwrap(),
                bet: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort_by_cached_key(|x| compute_rank(x.cards));
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, x)| (i + 1) as u32 * x.bet)
            .sum::<u32>(),
    )
}

fn compute_rank(input: &str) -> u32 {
    let mut max = 0;
    let mut second_max = 0;
    let mut counting_map = HashMap::new();
    let rank_number = input
        .chars()
        .map(|c| {
            *counting_map.entry(c).or_insert(0) += 1;
            if counting_map[&c] > max {
                max = counting_map[&c];
            } else if counting_map[&c] > second_max {
                second_max = counting_map[&c];
            }
            CARD_HEX[&c]
        })
        .collect::<String>();
    i32::from_str_radix(
        (value(max, second_max).to_string() + rank_number.as_str()).as_str(),
        16,
    )
    .unwrap() as u32
}

fn value(max: u32, second_max: u32) -> u32 {
    if max == 5 || max == 4 {
        max + 2
    } else if max == 3 {
        if second_max == 2 {
            5
        } else {
            4
        }
    } else if max == 2 {
        if second_max == 2 {
            3
        } else {
            2
        }
    } else {
        1
    }
}

fn compute_rank_2(input: &str) -> u32 {
    let mut max = 0;
    let mut second_max = 0;
    let mut counting_map = HashMap::new();
    let mut num_wildcard = 0;
    let rank_number = input
        .chars()
        .map(|c| {
            if c == 'J' {
                num_wildcard += 1;
                return '1';
            }
            *counting_map.entry(c).or_insert(0) += 1;
            if counting_map[&c] > max {
                max = counting_map[&c];
            } else if counting_map[&c] > second_max {
                second_max = counting_map[&c];
            }
            CARD_HEX_2[&c]
        })
        .collect::<String>();
    i32::from_str_radix(
        (value(max + num_wildcard, second_max).to_string() + rank_number.as_str()).as_str(),
        16,
    )
    .unwrap() as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|x| {
            let mut iter = x.split_whitespace();
            Hand {
                cards: iter.next().unwrap(),
                bet: iter.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<Hand>>();
    hands.sort_by_cached_key(|x| compute_rank_2(x.cards));
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, x)| (i + 1) as u32 * x.bet)
            .sum::<u32>(),
    )
}

advent_of_code::main!("07");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "07"));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "07"));
        assert_eq!(result, Some(5905));
    }
}

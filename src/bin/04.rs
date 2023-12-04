pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut iter = line.split(':').nth(1).unwrap().split('|').map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                });
                let win = iter.next().unwrap();
                let have = iter.next().unwrap();
                let points: u32 = have
                    .iter()
                    .map(|x| if win.contains(x) { 1 } else { 0 })
                    .sum();
                if points == 0 {
                    return 0;
                }
                2_i32.pow(points - 1) as u32
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut forward_vec = vec![1_u32; input.lines().count()];
    Some(
        input
            .lines()
            .map(|line| {
                let mut iter = line.split(':');
                let card_num = iter
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let mut iter2 = iter.next().unwrap().split('|').map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                });
                let win = iter2.next().unwrap();
                let have = iter2.next().unwrap();
                let points: u32 = have
                    .iter()
                    .map(|x| if win.contains(x) { 1 } else { 0 })
                    .sum();
                let number_cards = forward_vec[card_num as usize - 1];
                (0..points).for_each(|x| {
                    let card = card_num as usize + x as usize;
                    if card < forward_vec.len() {
                        forward_vec[card] += number_cards;
                    }
                });
                number_cards
            })
            .sum(),
    )
}

advent_of_code::main!("04");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "04"));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "04"));
        assert_eq!(result, Some(30));
    }
}

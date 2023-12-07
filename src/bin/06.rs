pub fn part_one(input: &str) -> Option<u32> {
    let mut iter = input.lines().map(|x| {
        x.split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });
    let times = iter.next().unwrap();
    let distances = iter.next().unwrap();

    Some(
        times
            .iter()
            .enumerate()
            .map(|(i, t)| {
                (1..*t)
                    .map(|x| if (t - x) * x > distances[i] { 1 } else { 0 })
                    .sum::<u32>()
            })
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut iter = input.lines().map(|x| {
        x.split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    });
    let time = iter.next().unwrap();
    let distance = iter.next().unwrap();

    Some(
        (1..time)
            .map(|x| if (time - x) * x > distance { 1 } else { 0 })
            .sum::<u32>(),
    )
}

advent_of_code::main!("06");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "06"));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "06"));
        assert_eq!(result, Some(71503));
    }
}

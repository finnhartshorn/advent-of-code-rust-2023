pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut game_num = 1;
    for line in input.lines() {
        let mut possible = true;
        let games: Vec<&str> = line.split(':').nth(1).unwrap().split(';').collect();
        for game in games {
            for draw in game.split(',') {
                let mut iter = draw.split_whitespace();
                let number = iter.next().unwrap().parse::<u32>().unwrap();
                let colour = iter.next().unwrap();
                match colour {
                    "red" => {
                        if number > 12 {
                            possible = false
                        }
                    }
                    "green" => {
                        if number > 13 {
                            possible = false
                        }
                    }
                    "blue" => {
                        if number > 14 {
                            possible = false
                        }
                    }
                    _ => println!("Error"),
                }
            }
        }

        if possible {
            sum += game_num;
        }
        game_num += 1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let games = line.split(':').nth(1).unwrap().split([',', ';']);
        for game in games {
            let mut iter = game.split_whitespace();
            let number = iter.next().unwrap().parse::<u32>().unwrap();
            let colour = iter.next().unwrap();
            match colour {
                "red" => {
                    if number > r {
                        r = number
                    }
                }
                "green" => {
                    if number > g {
                        g = number
                    }
                }
                "blue" => {
                    if number > b {
                        b = number
                    }
                }
                _ => println!("Error"),
            }
        }
        sum += r * g * b;
    }
    Some(sum)
}

advent_of_code::main!("02");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", "02"));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", "02"));
        assert_eq!(result, Some(2286));
    }
}

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let matches: Vec<&str> = line.matches(char::is_numeric).collect();
            matches
                .first()
                .and_then(|a| matches.last().map(|b| format!("{}{}", a, b)))
                .and_then(|c| c.parse::<u32>().ok())
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();

    let mut answer = 0;

    for line in lines {
        let matches: Vec<(usize, &str)> = match_list(
            line,
            vec![
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2",
                "3", "4", "5", "6", "7", "8", "9",
            ],
        );

        let mut cloned = matches.clone();
        cloned.sort_by_key(|k| k.0);

        let f = cloned.first().unwrap().1;
        let l = cloned.last().unwrap().1;

        let c = format!("{}{}", parse_value(f), parse_value(l));
        answer += c.parse::<u32>().unwrap();
    }

    Some(answer)
}

fn match_list<'a>(input: &'a str, pattern: Vec<&'a str>) -> Vec<(usize, &'a str)> {
    pattern
        .iter()
        .map(|s| (input.find(*s), *s))
        .filter(|v| v.0.is_some())
        .map(|v| (v.0.unwrap(), v.1))
        .collect()
}

fn parse_value(input: &str) -> &str {
    match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        value => value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}

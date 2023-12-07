use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space0, space1};
use nom::IResult;
use nom::multi::separated_list1;
use regex::Regex;

#[derive(Debug, Default, PartialEq, Clone)]
struct Game {
    pub id: u32,
    pub rounds: Vec<Round>,
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Round {
    pub red: Option<u32>,
    pub green: Option<u32>,
    pub blue: Option<u32>,
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    Ok((input, id.parse().unwrap()))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, colors) = separated_list1(
        tag(","),
        parse_color,
    )(input)?;

    let mut round: Round = Round { red: None, green: None, blue: None };

    colors.into_iter()
        .for_each(|(color, val)| match color {
            "red" => round.red = Some(val),
            "green" => round.green = Some(val),
            "blue" => round.blue = Some(val),
            _ => panic!("unknown color")
        });

    Ok((input, round))
}


fn parse_color(input: &str) -> IResult<&str, (&str, u32)> {
    let (input, _) = space0(input)?;
    let (input, val) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alpha1(input)?;
    Ok((input, (color, val.parse().unwrap())))
}


fn parse_rounds(input: &str) -> IResult<&str, Vec<Round>> {
    let rounds = input
        .split(";")
        .map(|s| parse_round(s).unwrap().1)
        .collect();
    Ok((input, rounds))
}


fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = parse_game_id(input)?;
    let (input, rounds) = parse_rounds(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let games = input
        .lines()
        .map(|s| parse_game(s).unwrap().1)
        .collect();
    Ok((input, games))
}

pub fn run(input: &str) -> u32 {
    let (_, games) = parse_games(input).unwrap();
    games
        .iter()
        .filter(|g| g.rounds
            .iter()
            .all(|r| r.red.unwrap_or_default() <= 12 as u32 && r.green.unwrap_or_default() <= 13 as u32 && r.blue.unwrap_or_default() <= 14 as u32))
        .map(|g|g.id)
        .sum()
}

pub fn naive(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut game = line.split(":");
        let id = game.next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();
        let mut rounds = game.next().unwrap().split(";");
        let mut valid = true;
        for round in rounds {
            let mut colors = round.split(",");
            for color in colors {
                let mut color = color.trim().split(" ");
                let val = color.next().unwrap().parse::<u32>().unwrap();
                let color = color.next().unwrap();
                match color {
                    "red" => if val > 12 { valid = false },
                    "green" => if val > 13 { valid = false },
                    "blue" => if val > 14 { valid = false },
                    _ => panic!("unknown color")
                }
            }
        }
        if valid {
            sum += id;
        }
    }
    sum
}

pub fn regexp(input: &str) -> u32 {
    let mut sum = 0;
    let re = Regex::new("(?P<qty>[0-9]+) (?P<color>(red|blue|green))").unwrap();

    for line in input.lines() {
        let mut game = line.split(":");
        let id = game.next().unwrap().split(" ").last().unwrap().parse::<u32>().unwrap();

        let mut rounds = game.next().unwrap().split(";");
        let mut valid = true;

        for round in rounds {
            let mut colors = round.split(",");
            for color in colors {

                let caps = re.captures(color.trim()).unwrap();

                let val = &caps["qty"].parse::<u32>().unwrap();
                let color = &caps["color"];
                match color {
                    "red" => if 12 < *val  { valid = false },
                    "green" => if 13 < *val { valid = false },
                    "blue" => if 14 < *val { valid = false },
                    _ => panic!("unknown color")
                }
            }
        }

        if valid {
            sum += id;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_game() {
        let input = "Game 10: 4 red, 1 green, 4 blue; 7 green, 8 blue, 4 red; 9 green, 3 red, 8 blue; 5 red, 2 green, 7 blue";
        let result = parse_game(input).unwrap();
        let expected = Game {
            id: 10,
            rounds: vec![
                Round { red: Some(4), green: Some(1), blue: Some(4) },
                Round { red: Some(4), green: Some(7), blue: Some(8) },
                Round { red: Some(3), green: Some(9), blue: Some(8) },
                Round { red: Some(5), green: Some(2), blue: Some(7) },
            ],
        };
        assert_eq!(result.1, expected);
    }

    #[test]
    fn test_N_games() {
        let input = "Game 10: 4 red, 1 green, 4 blue; 7 green, 8 blue, 4 red; 9 green, 3 red, 8 blue; 5 red, 2 green, 7 blue\n\
        Game 10: 4 red, 1 green, 4 blue; 7 green, 8 blue, 4 red; 9 green, 3 red, 8 blue; 5 red, 2 green, 7 blue";

        let result = parse_games(input).unwrap();
        let game = Game {
            id: 10,
            rounds: vec![
                Round { red: Some(4), green: Some(1), blue: Some(4) },
                Round { red: Some(4), green: Some(7), blue: Some(8) },
                Round { red: Some(3), green: Some(9), blue: Some(8) },
                Round { red: Some(5), green: Some(2), blue: Some(7) },
            ],
        };
        let expected = vec![game.clone(), game];
        assert_eq!(result.1, expected);
    }

    #[test]
    fn solution_1() {
        let input = include_str!("../input/parts/2.1").into();
        dbg!(run(input));
    }

    #[test]
    fn solution_1_naive() {
        let input = include_str!("../input/parts/2.1").into();
        dbg!(naive(input));
    }
    #[test]
    fn solution_1_regexp() {
        let input = include_str!("../input/parts/2.1").into();
        dbg!(regexp(input));
    }
}
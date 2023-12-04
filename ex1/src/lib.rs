
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{peek, value},
    IResult,
    multi::many0,
};

pub fn naive_1(input: String) -> u32 {
    let sum = input.split("\n")
        .map(|s| {
            let numbers = s.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            let mut ss: String = numbers.first().unwrap().to_string();
            ss.push_str(numbers.last().unwrap().to_string().as_str());
            let cat = ss.parse::<u32>().unwrap();
            //dbg!(cat)
            cat
        }
        ).sum();
    //dbg!(sum)
    sum
}

pub fn run(input: &str) -> i32 {
    let res = input.lines().map(|s| parser(s)).sum();
    //dbg!(res)
    res
}

fn parse_str_num(i: &str) -> IResult<&str, &str> {

    let (tail, out) = peek(alt((
        value("1", tag("one")),
        value("2", tag("two")),
        value("3", tag("three")),
        value("4", tag("four")),
        value("5", tag("five")),
        value("6", tag("six")),
        value("7", tag("seven")),
        value("8", tag("eight")),
        value("9", tag("nine")),
    )))(i)?;
    let (tail, _) = take(1usize)(tail)?;
    Ok((tail, out))
}
fn parse_num(i: &str) -> IResult<&str, Vec<&str>> {

    many0(alt((
        take_while1(|c: char| c.is_numeric()),
        parse_str_num,
        value("", take(1usize)),
    )))(i)
}

fn parser(i: &str) -> i32 {
    let (_, num) = parse_num(i).expect("number not returned");
    let binding = num.join("");
    let mut chars = binding.trim().chars();
    let first = chars.next().unwrap_or('0');
    let last = chars.next_back().unwrap_or(first);
    let val = format!("{}{}", first, last).parse::<i32>().unwrap();
    val
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".into();
        assert_eq!(run(input), 142);
    }

    #[test]
    fn solution_1_naive() {
        let input = include_str!("../input/parts/1.1").into();
        dbg!(naive_1(input));
    }

    #[test]
    fn test_parsing() {
        let input = "one2four";
        let n = parser(input);
        assert_eq!(n, 14);
    }

    #[test]
    fn test_parsing_multiline() {
        let input = "2one2four\n2one2four";
        let n = run(input);
        assert_eq!(n, 48);
    }

    #[test]
    fn test_parsing_multiline_extra() {
        let input = "one2four\none2four";
        let n = run(input);
        assert_eq!(n, 28);
    }


    #[test]
    fn solution_1() {
        let input = include_str!("../input/parts/1.1").into();
        dbg!(naive_1(input));
    }

    #[test]
    fn solution_2() {
        let input = include_str!("../input/parts/1.2").into();
        dbg!(run(input));
    }
}
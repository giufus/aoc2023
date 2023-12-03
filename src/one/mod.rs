use std::ops::Add;

pub fn one(input: String) -> u32 {
    let sum = input.split("\n")
        .map(|s| {
            let numbers = s.chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();

            let mut ss: String = numbers.first().unwrap().to_string();
            ss.push_str(numbers.last().unwrap().to_string().as_str());
            let cat = ss.parse::<u32>().unwrap();
            dbg!(cat)
        }
        ).sum();
    dbg!(sum)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".into();
        assert_eq!(one(input), 142);
    }


}
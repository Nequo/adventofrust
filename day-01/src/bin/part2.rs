fn main() {
    println!("part2");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut maybedigit: Vec<char> = vec![];
        let mut digits: Vec<u32> = vec![];
        let mut index = 0;
        for char in line.chars() {
            if char.is_ascii_digit() {
                digits.push(char.to_digit(10).unwrap());
            }
            else {
                maybedigit.push(char);
            }
            // The idea here is that maybedigit is an array of all non-digits encountered in the
            // line so far. On every pass of the loop, we check for matches at the
            // start of the string, moving a pointer "i" to try all possible string slices starting
            // from "i" and going to the end of the array.
            // a d o n e
            // ^
            // "adone" does not match any number strings, so we increment "i" by 1 and try again:
            // a d o n e
            //   ^
            // "done" does not match any number strings, so we increment "i" by 1 and try again:
            // a d o n e
            //     ^ 
            // one matches 1, so we add 1 to our digits array and we set "index" to start at 3 which
            // is the position of the character "n" for our next search. This is like consuming the
            // token, except we shouldnt consume all of it, just the first character. This is to 
            // deal with cases like "3twone" where the solution expects 1 as the last digit. If we
            // had consumed "two", then the only charecters left are "ne" which won't match "one".
            for i in index..maybedigit.len() {
                let s : String = maybedigit[i..].iter().collect::<String>();
                let d = {
                    if s.starts_with("one") { Some(1) }
                    else if s.starts_with("two") { Some(2) }
                    else if s.starts_with("three") { Some(3) }
                    else if s.starts_with("four") { Some(4) }
                    else if s.starts_with("five") { Some(5) }
                    else if s.starts_with("six") { Some(6) }
                    else if s.starts_with("seven") { Some(7) }
                    else if s.starts_with("eight") { Some(8) }
                    else if s.starts_with("nine") { Some(9) }
                    else { None }
                };
                if d.is_some() {
                    digits.push(d.unwrap());
                    index = i+1;
                }
            }
        }
        
        let new = digits.first().unwrap() * 10 + digits.last().unwrap();
        sum += new;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one () {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281".to_string());
        let result2 = part2("two1nine");
        assert_eq!(result2, "29".to_string());
        let result3 = part2("nineninesix6nine");
        assert_eq!(result3, "99".to_string());
    }
}

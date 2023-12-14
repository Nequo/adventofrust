fn main() {
    println!("part1");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits: Vec<u32> = vec![];
        for char in line.chars() {
            if char.is_ascii_digit() {
                digits.push(char.to_digit(10).unwrap());
            }
        }
        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one () {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142".to_string());
    }
}

fn main() {
    println!("part1");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    "todo".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one () {
        let result = part1("");
        assert_eq!(result, "4".to_string());
    }
}

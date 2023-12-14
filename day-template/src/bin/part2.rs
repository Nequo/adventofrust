fn main() {
    println!("part2");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    "todo".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one () {
        let result = part2("");
        assert_eq!(result, "2".to_string());
    }
}

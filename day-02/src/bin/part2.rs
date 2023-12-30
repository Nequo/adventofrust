fn main() {
    println!("part2");
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let sets = line.rsplit_once(':').unwrap().1;
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;
        for set in sets.split(';') {
            for draw in set.split(',') {
                let (amount, color) = draw.trim().split_once(' ').unwrap();
                match (color, amount) {
                    ("red", x ) => if x.parse::<i32>().unwrap() > max_red {max_red = x.parse::<i32>().unwrap()},
                    ("green", x ) => if x.parse::<i32>().unwrap() > max_green {max_green = x.parse::<i32>().unwrap()},
                    ("blue", x ) => if x.parse::<i32>().unwrap() > max_blue {max_blue = x.parse::<i32>().unwrap()}
                    _ => panic!("Invalid line")
                };
            }
        }
        sum += max_red * max_green * max_blue;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one () {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286".to_string());
    }
}

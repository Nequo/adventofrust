fn main() {
    println!("part1");
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (game,sets) = line.rsplit_once(':').unwrap();
        let game_id = game.trim().split_once(' ').unwrap().1;
        let mut valid_game = false;
        'outer: for set in sets.split(';') {
            for draw in set.split(',') {
                let (amount, color) = draw.trim().split_once(' ').unwrap();
                valid_game = match (color, amount) {
                    ("red", x ) => if x.parse::<i32>().unwrap() > 12 {false} else {true},
                    ("green", x ) => if x.parse::<i32>().unwrap() > 13 {false} else {true},
                    ("blue", x ) => if x.parse::<i32>().unwrap() > 14 {false} else {true},
                    _ => panic!("Invalid line")
                };
                if !valid_game {
                    break 'outer;
                }
            }
        }
        if valid_game {
            sum += game_id.parse::<i32>().unwrap();
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one () {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "8".to_string());
    }
}

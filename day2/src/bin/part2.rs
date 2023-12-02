use anyhow::{anyhow, Result};
use regex::Regex;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 2, part 2 result: {}", result);
    Ok(())
}

struct Scanner {
    s: String,
}

impl Scanner {
    fn new(s: String) -> Self {
        Scanner { s }
    }

    fn consume_id(&mut self) -> Result<i64> {
        self.consume_string("Game ".into())?;
        self.consume_int()
    }

    fn consume_turn(&mut self) -> Result<Turn> {
        let mut go = true;
        let color_re = Regex::new(r"^(blue|green|red)")?;
        let separator_re = Regex::new(r"^(,|;)")?;

        let mut turn = Turn {
            blue: 0,
            red: 0,
            green: 0,
        };

        while go {
            self.consume_whitespace()?;
            let count = self.consume_int()?;
            self.consume_whitespace()?;
            let color = self.consume_regex(color_re.clone())?;

            match color.as_ref() {
                "blue" => turn.blue += count,
                "green" => turn.green += count,
                "red" => turn.red += count,
                _ => unreachable!(),
            }

            if self.s.is_empty() {
                go = false;
                continue;
            }

            let sep = self.consume_regex(separator_re.clone())?;
            if sep == ";" {
                go = false;
            }
        }

        Ok(turn)
    }

    fn consume_string(&mut self, s: String) -> Result<String> {
        if self.s.starts_with(&s) {
            self.s = self.s.strip_prefix(&s).unwrap().to_owned();
            Ok(s)
        } else {
            Err(anyhow!("Invalid consume: {}", s))
        }
    }

    fn consume_int(&mut self) -> Result<i64> {
        let re = Regex::new(r"^(\d+)")?;
        let d = self.consume_regex(re)?;
        Ok(d.parse::<i64>().unwrap())
    }

    fn consume_whitespace(&mut self) -> Result<()> {
        let re = Regex::new(r"^(\s+)")?;
        self.consume_regex(re)?;
        Ok(())
    }

    fn consume_regex(&mut self, re: Regex) -> Result<String> {
        if self.s.is_empty() {
            return Err(anyhow!("end"));
        }
        let caps = re.captures(&self.s).unwrap();
        let c = caps.get(0).unwrap().as_str().into();
        self.s = self.s.strip_prefix(&c).unwrap().to_owned();
        Ok(c)
    }
}

struct Game {
    power: i64,
}

struct Turn {
    blue: i64,
    red: i64,
    green: i64,
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let mut scanner = Scanner::new(value);

        let _id = scanner.consume_id().unwrap();
        scanner.consume_string(":".into()).unwrap();

        let mut go = true;
        let mut turns: Vec<Turn> = vec![];
        while go {
            match scanner.consume_turn() {
                Ok(t) => turns.push(t),
                Err(_) => go = false,
            }
        }

        let found_max_blue = turns.iter().map(|t| t.blue).max().unwrap();
        let found_max_red = turns.iter().map(|t| t.red).max().unwrap();
        let found_max_green = turns.iter().map(|t| t.green).max().unwrap();

        let power = found_max_blue * found_max_red * found_max_green;

        Game { power }
    }
}

fn solve(lines: Vec<String>) -> i64 {
    let mut total = 0;
    for line in lines {
        let game: Game = line.into();
        total += game.power;
    }
    total
}

fn lines(path: String) -> Result<Vec<String>> {
    let input_data: String = String::from_utf8(std::fs::read(path)?)?;
    let l: Vec<String> = input_data
        .trim()
        .split('\n')
        .map(|input| input.trim().to_string())
        .filter(|input| input != "")
        .collect();
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let expected = 2286;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

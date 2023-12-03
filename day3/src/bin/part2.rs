use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 3, part 2 result: {}", result);
    Ok(())
}

#[derive(Debug)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn adjacent(&self, other: &Coordinate) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

struct Number {
    v: i64,
    coordinates: Vec<Coordinate>,
}

impl Number {
    fn adjacent(&self, coordinate: &Coordinate) -> bool {
        self.coordinates.iter().any(|c| c.adjacent(coordinate))
    }
}

struct Symbol {
    s: String,
    coordinate: Coordinate,
}

fn solve(lines: Vec<String>) -> i64 {
    let digits = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let symbol_strings = vec![
        "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "-", "+", "=", "~", "`", "/", "?",
    ];

    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    for (y, line) in lines.iter().enumerate() {
        let mut num: Option<Number> = None;

        for (x, ch) in line.chars().enumerate() {
            let cs = ch.to_string();
            if !digits.contains(&cs.as_str()) && num.is_some() {
                numbers.push(num.unwrap());
                num = None;
            }

            if digits.contains(&cs.as_str()) {
                if num.is_none() {
                    num = Some(Number {
                        v: 0,
                        coordinates: vec![],
                    })
                }

                num.as_mut().unwrap().v *= 10;
                num.as_mut().unwrap().v += cs.parse::<i64>().unwrap();
                num.as_mut().unwrap().coordinates.push(Coordinate {
                    x: x as i64,
                    y: y as i64,
                });

                continue;
            }

            if symbol_strings.contains(&cs.as_str()) {
                symbols.push(Symbol {
                    s: cs.clone(),
                    coordinate: Coordinate {
                        x: x as i64,
                        y: y as i64,
                    },
                });

                continue;
            }

            if ch == '.' {
                continue;
            }

            panic!("Unknown character: '{}'", ch);
        }

        if num.is_some() {
            numbers.push(num.unwrap());
        }
    }

    let mut total = 0;
    for symbol in symbols {
        if symbol.s != "*" {
            continue;
        }

        let mut adjacent_count = 0;
        let mut adjacent_product = 1;

        for number in &numbers {
            if number.adjacent(&symbol.coordinate) {
                adjacent_count += 1;
                adjacent_product *= number.v;
            }
        }

        if adjacent_count == 2 {
            total += adjacent_product;
        }
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
        let expected = 467835;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

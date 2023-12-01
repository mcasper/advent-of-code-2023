use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 1, part 2 result: {}", result);
    Ok(())
}

fn solve(lines: Vec<String>) -> i64 {
    let mut numbers = vec![];
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut substrs = vec![];

    for line in lines {
        let mut first_number = -1;
        let mut last_number = -1;

        let chars = line.split("");
        for c in chars {
            if let Ok(i) = c.parse::<i64>() {
                if first_number == -1 {
                    first_number = i;
                }

                last_number = i;
                substrs.clear();
            } else {
                let mut new_substrs: Vec<String> =
                    substrs.iter().map(|s| format!("{}{}", s, c)).collect();
                new_substrs.push(c.to_owned());

                substrs.clear();
                for substr in new_substrs {
                    let mut matched = false;

                    for (i, word) in words.iter().enumerate() {
                        if word.starts_with(&substr) {
                            matched = true;
                            if word == &substr {
                                let ii = i + 1;

                                if first_number == -1 {
                                    first_number = ii as i64;
                                }

                                last_number = ii as i64;
                                matched = false;
                            }
                        }
                    }

                    if matched {
                        substrs.push(substr);
                    }
                }
            }
        }

        println!(
            "Found {} and {} for line {}",
            first_number, last_number, line
        );

        if first_number == -1 || last_number == -1 {
            panic!("failed to find first or last number!")
        }

        numbers.push((first_number * 10) + last_number)
    }

    numbers.iter().sum()
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
        let expected = 281;
        let actual = solve(lines("src/bin/sample2.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

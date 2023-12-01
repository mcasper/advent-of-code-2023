use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 1, part 1 result: {}", result);
    Ok(())
}

fn solve(lines: Vec<String>) -> i64 {
    let mut numbers = vec![];

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
            }
        }

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
        let expected = 142;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

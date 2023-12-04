use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 4, part 1 result: {}", result);
    Ok(())
}

fn solve(lines: Vec<String>) -> i64 {
    let mut result = 0;

    for line in lines {
        let mut score = 0;
        let mut winning: Vec<i64> = vec![];
        let content = line.split(":").collect::<Vec<&str>>()[1];
        let sides = content.split("|").collect::<Vec<&str>>();

        for t in sides[0].split(" ") {
            if t.trim() == "" {
                continue;
            }

            winning.push(t.parse::<i64>().unwrap());
        }

        for t in sides[1].split(" ") {
            if t.trim() == "" {
                continue;
            }

            let n = t.parse::<i64>().unwrap();
            if winning.contains(&n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        result += score;
    }

    result
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
        let expected = 13;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 4, part 2 result: {}", result);
    Ok(())
}

fn solve(lines: Vec<String>) -> i64 {
    let mut scorecard_count: HashMap<usize, i64> = HashMap::new();

    for (i, _) in lines.iter().enumerate() {
        scorecard_count.insert(i + 1, 1);
    }

    for (i, line) in lines.iter().enumerate() {
        let card_number = i + 1;
        let this_card_count = scorecard_count.get(&card_number).unwrap();

        let mut winners = 0;
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
                winners += 1;
            }
        }

        for _ in 0..*this_card_count {
            for i in 0..winners {
                if let Some(v) = scorecard_count.get_mut(&(i + 2)) {
                    println!("incrementing");
                    *v += 1;
                }
            }
        }

        println!(
            "Card {} had {} copies and {} winners, counts: {:?}",
            card_number, this_card_count, winners, scorecard_count
        )
    }

    let mut result = 0;
    for (_, v) in scorecard_count {
        result += v;
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
        let expected = 30;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

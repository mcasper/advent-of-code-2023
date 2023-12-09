use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 9, part 1 result: {}", result);
    Ok(())
}

fn solve(lines: Vec<String>) -> i64 {
    let mut sum = 0;

    for line in lines {
        let mut current_nums = line
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let mut runs: Vec<Vec<i64>> = vec![current_nums.clone()];
        let mut decomposing = true;

        while decomposing {
            let mut differences: Vec<i64> = vec![];
            for (i, num) in current_nums.iter().enumerate() {
                let next_num = current_nums.get(i + 1);
                if next_num.is_none() {
                    continue;
                }

                differences.push(next_num.unwrap() - num);
            }

            if differences.iter().all(|n| n == &0) {
                decomposing = false;
            }

            current_nums = differences.clone();
            runs.push(differences);
        }

        runs.reverse();

        let mut next_number = 0;
        for (i, run) in runs.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let last_number = run.last().unwrap();
            next_number += last_number;
        }

        sum += next_number
    }

    sum
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
        let expected = 114;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

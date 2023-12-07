use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 6, part 1 result: {}", result.unwrap());
    Ok(())
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let times_line = lines.first().unwrap();
    let distances_line = lines.last().unwrap();

    let times = times_line
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let distances = distances_line
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut margin = 1;

    for (i, time) in times.iter().enumerate() {
        let mut winners = 0;
        let distance = distances.get(i).unwrap();

        for millimeters_per_second in 0..(time + 1) {
            let remaining_time = time - millimeters_per_second;
            let travelled = remaining_time * millimeters_per_second;
            if travelled > *distance {
                winners += 1;
            }
        }

        margin *= winners;
    }

    Ok(margin)
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
        let expected = 288;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}

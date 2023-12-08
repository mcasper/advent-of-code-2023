use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 8, part 1 result: {}", result);
    Ok(())
}

struct Node {
    left: String,
    right: String,
}

fn solve(lines: Vec<String>) -> i64 {
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let pattern: Vec<char> = lines[0].chars().collect();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let parts = line.split(" = ").collect::<Vec<&str>>();
        let children = parts[1]
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();

        nodes.insert(
            parts[0].to_owned(),
            Node {
                left: children[0].to_owned(),
                right: children[1].to_owned(),
            },
        );
    }

    let mut steps = 0;
    let mut current_node_ident = "AAA".to_owned();
    let mut pattern_index = 0;

    while current_node_ident != "ZZZ" {
        if pattern_index >= pattern.len() {
            pattern_index = 0;
        }

        let current_node = nodes.get(&current_node_ident).unwrap();
        match pattern.get(pattern_index) {
            Some('L') => {
                current_node_ident = current_node.left.clone();
            }
            Some('R') => {
                current_node_ident = current_node.right.clone();
            }
            _ => unreachable!("unknown pattern value"),
        }

        pattern_index += 1;
        steps += 1;
    }

    steps
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
        let expected = 2;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve2() {
        let expected = 6;
        let actual = solve(lines("src/bin/sample2.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

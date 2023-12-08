use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 8, part 2 result: {}", result);
    Ok(())
}

struct Node {
    left: String,
    right: String,
}

fn lcm(nums: Vec<i64>) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].to_vec());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
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

    let node_idents = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let mut cycle_lengths: Vec<i64> = vec![];

    for ident in &node_idents {
        let mut steps = 0;
        let mut pattern_index = 0;
        let mut go = true;
        let mut current_ident = ident.to_owned().clone();

        let mut cycle_length = 0;

        while go {
            steps += 1;

            if pattern_index >= pattern.len() {
                pattern_index = 0;
            }

            let current_node = nodes.get(&current_ident).unwrap();

            match pattern.get(pattern_index) {
                Some('L') => {
                    current_ident = current_node.left.clone();
                }
                Some('R') => {
                    current_ident = current_node.right.clone();
                }
                _ => unreachable!("unknown pattern value"),
            }

            pattern_index += 1;

            if current_ident.ends_with("Z") {
                if cycle_length == 0 {
                    cycle_length = steps;
                    continue;
                }

                if (steps - cycle_length) == cycle_length {
                    go = false;
                }
            }
        }

        cycle_lengths.push(cycle_length);
    }

    lcm(cycle_lengths)
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
    fn test_solve3() {
        let expected = 6;
        let actual = solve(lines("src/bin/sample3.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 12, part 1 result: {}", result);
    Ok(())
}

#[derive(Clone, Debug)]
struct Report {
    springs: Vec<char>,
    broken_segments: Vec<usize>,
}

impl From<&String> for Report {
    fn from(value: &String) -> Self {
        let parts = value.split(" ").collect::<Vec<&str>>();
        Report {
            springs: parts[0].chars().collect(),
            broken_segments: parts[1]
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn possibles(cs: Vec<char>, segments: Vec<usize>) -> i64 {
    if segments.is_empty() {
        if cs.iter().all(|c| c != &'#') {
            return 1;
        } else {
            return 0;
        }
    }

    if cs.is_empty() {
        return 0;
    }

    if segments.first().unwrap() > &cs.len() {
        return 0;
    }

    let ways = match (cs.first(), segments.first()) {
        (Some('.'), _) => possibles(cs[1..cs.len()].to_vec(), segments),
        (Some('#'), Some(l)) => {
            if cs[0..*l].iter().all(|c| c == &'#' || c == &'?')
                && cs.get(*l).unwrap_or(&'a') != &'#'
            {
                let remaining = if *l >= cs.len() {
                    vec![]
                } else {
                    cs[*l + 1..cs.len()].to_vec()
                };
                possibles(remaining, segments[1..segments.len()].to_vec())
            } else {
                0
            }
        }
        (Some('?'), Some(l)) => {
            if cs[0..*l].iter().all(|c| c == &'#' || c == &'?')
                && cs.get(*l).unwrap_or(&'a') != &'#'
            {
                let remaining = if *l >= cs.len() {
                    vec![]
                } else {
                    cs[*l + 1..cs.len()].to_vec()
                };

                possibles(remaining, segments[1..segments.len()].to_vec())
                    + possibles(cs[1..cs.len()].to_vec(), segments)
            } else {
                possibles(cs[1..cs.len()].to_vec(), segments)
            }
        }
        (None, _) => 0,
        (_, None) => 0,
        (a, b) => unreachable!("unhandled case {:?},{:?}", a, b),
    };
    // cache
    ways
}

fn solve(lines: Vec<String>) -> i64 {
    let reports: Vec<Report> = lines.iter().map(|l| l.into()).collect();
    let mut configs = 0;

    for report in reports {
        let count = possibles(report.springs.clone(), report.broken_segments.clone());
        configs += count;
    }

    configs
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
        let expected = 21;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

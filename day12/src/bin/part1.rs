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

struct ConfigState {
    spring_index: usize,
    remaining_segments: Vec<usize>,
    indexes: Vec<(usize, usize)>,
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

// 4321 - too low
// 9109 - too big

fn solve(lines: Vec<String>) -> i64 {
    let reports: Vec<Report> = lines.iter().map(|l| l.into()).collect();
    let mut configurations: Vec<Vec<(usize, usize)>> = vec![];

    for (i, report) in reports.iter().enumerate() {
        let mut states = vec![ConfigState {
            spring_index: 0,
            remaining_segments: report.broken_segments.clone(),
            indexes: vec![],
        }];

        let mut report_count = 0;

        while states.len() > 0 {
            let mut next_states: Vec<ConfigState> = vec![];

            for state in &states {
                if state.remaining_segments.is_empty() {
                    if report.springs.iter().enumerate().all(|(ci, c)| {
                        if c == &'#' {
                            state
                                .indexes
                                .iter()
                                .any(|(start, end)| start <= &ci && end >= &ci)
                            // check to make sure it's covered
                        } else {
                            true
                        }
                    }) {
                        configurations.push(state.indexes.clone());
                        report_count += 1;
                    }
                    continue;
                }
                if state.spring_index >= report.springs.len() {
                    continue;
                }

                next_states.push(ConfigState {
                    spring_index: state.spring_index + 1,
                    remaining_segments: state.remaining_segments.clone(),
                    indexes: state.indexes.clone(),
                });

                let next_char = report.springs[state.spring_index];
                if next_char == '?' || next_char == '#' {
                    let next_length = state.remaining_segments.first().unwrap();
                    if state.spring_index + next_length > report.springs.len() {
                        continue;
                    }
                    if (state.spring_index..(state.spring_index + next_length))
                        .all(|i| report.springs[i] == '?' || report.springs[i] == '#')
                    {
                        if state.spring_index + next_length < report.springs.len()
                            && report.springs[state.spring_index + next_length] == '#'
                        {
                            continue;
                        }

                        if state.spring_index > 0 && report.springs[state.spring_index - 1] == '#' {
                            continue;
                        }

                        let mut new_indexes = state.indexes.clone();
                        new_indexes
                            .push((state.spring_index, state.spring_index + next_length - 1));
                        next_states.push(ConfigState {
                            spring_index: state.spring_index + next_length + 1,
                            remaining_segments: state.remaining_segments
                                [1..state.remaining_segments.len()]
                                .to_vec(),
                            indexes: new_indexes,
                        });
                    }
                }
            }

            states = next_states;
        }

        // println!("Report {} has {} valid configurations", i, report_count);
    }

    configurations.dedup();
    configurations.len() as i64
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

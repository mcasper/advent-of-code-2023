use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 5, part 1 result: {}", result);
    Ok(())
}

#[derive(Clone, Debug)]
struct MapRange {
    dest_start: i64,
    source_start: i64,
    size: i64,
}

#[derive(Clone, Debug)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn new(from: String, to: String) -> Self {
        Self {
            from,
            to,
            ranges: vec![],
        }
    }

    fn add_range(&mut self, dest_start: i64, source_start: i64, range: i64) {
        self.ranges.push(MapRange {
            dest_start,
            source_start,
            size: range,
        })
    }

    fn resolve(&self, n: i64) -> i64 {
        for range in &self.ranges {
            if n >= range.source_start && n <= (range.source_start + range.size) {
                let offset = n - range.source_start;
                return range.dest_start + offset;
            }
        }

        n
    }
}

fn find_location(seed: i64, maps: Vec<Map>) -> Result<i64> {
    let mut current_number = seed;
    let mut current_kind = "seed";

    while current_kind != "location" {
        for map in &maps {
            if map.from == current_kind {
                current_kind = &map.to;
                current_number = map.resolve(current_number);
                break;
            }
        }
    }

    Ok(current_number)
}

fn solve(lines: Vec<String>) -> i64 {
    let mut seeds = vec![];
    let mut maps = vec![];

    let mut pending_map: Option<Map> = None;
    for line in lines {
        if line.starts_with("seeds: ") {
            let temp = line.strip_prefix("seeds: ").unwrap();
            seeds = temp.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
            continue;
        }

        if line.is_empty() {
            if let Some(m) = pending_map {
                maps.push(m);
            }
            pending_map = None;
            continue;
        }

        if line.contains("map:") {
            if let Some(m) = pending_map {
                maps.push(m);
            }

            let parts = line
                .strip_suffix(" map:")
                .unwrap()
                .split("-to-")
                .collect::<Vec<&str>>();
            pending_map = Some(Map::new(parts[0].to_owned(), parts[1].to_owned()));
            continue;
        }

        let nums = line
            .split(" ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let map = pending_map.as_mut().unwrap();
        map.add_range(nums[0], nums[1], nums[2]);
    }

    if let Some(m) = pending_map {
        maps.push(m);
    }

    let mut lowest_seed_number = 9999999999;
    for seed in seeds {
        let location_number = find_location(seed, maps.clone()).unwrap();
        if location_number < lowest_seed_number {
            lowest_seed_number = location_number;
        }
    }

    lowest_seed_number
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
        let expected = 35;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

use std::{collections::HashMap, thread, time::Instant};

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 5, part 2 result: {}", result);
    Ok(())
}

#[derive(Clone, Debug)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    size: u64,
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

    fn add_range(&mut self, dest_start: u64, source_start: u64, range: u64) {
        self.ranges.push(MapRange {
            dest_start,
            source_start,
            size: range,
        })
    }

    fn resolve(&self, n: u64) -> u64 {
        for range in &self.ranges {
            if n >= range.source_start && n < (range.source_start + range.size) {
                let offset = n - range.source_start;
                return range.dest_start + offset;
            }
        }

        n
    }
}

fn find_location(seed: u64, maps: HashMap<String, Map>) -> Result<u64> {
    let mut current_number = seed;
    let mut current_kind = "seed";

    while current_kind != "location" {
        let map = maps.get(current_kind).unwrap();
        current_kind = &map.to;
        current_number = map.resolve(current_number);
    }

    Ok(current_number)
}

fn solve(lines: Vec<String>) -> u64 {
    let mut seeds = vec![];
    let mut maps: HashMap<String, Map> = HashMap::new();

    let mut pending_map: Option<Map> = None;
    for line in lines {
        if line.starts_with("seeds: ") {
            let temp = line.strip_prefix("seeds: ").unwrap();
            seeds = temp.split(" ").map(|s| s.parse::<u64>().unwrap()).collect();
            continue;
        }

        if line.is_empty() {
            if let Some(m) = pending_map {
                maps.insert(m.from.clone(), m);
            }
            pending_map = None;
            continue;
        }

        if line.contains("map:") {
            if let Some(m) = pending_map {
                maps.insert(m.from.clone(), m);
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
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let map = pending_map.as_mut().unwrap();
        map.add_range(nums[0], nums[1], nums[2]);
    }

    if let Some(m) = pending_map {
        maps.insert(m.from.clone(), m);
    }

    let mut threads = vec![];
    let mut i = 0;

    for chunk in seeds.chunks(2) {
        let ii = i.clone();
        let start = chunk[0].clone();
        let size = chunk[1].clone();
        let mm = maps.clone();
        let start_time = Instant::now();
        threads.push(thread::spawn(move || {
            let mut lowest_seed_number = 9999999999;
            let mut i = start;
            let end = i + size;
            let mut done = 0;
            while i < end {
                if done % 10000000 == 0 {
                    println!(
                        "[t{}][{}s]: Got through {}/{}",
                        ii,
                        start_time.elapsed().as_secs(),
                        done,
                        size
                    );
                }
                let location_number = find_location(i, mm.clone()).unwrap();
                if location_number < lowest_seed_number {
                    lowest_seed_number = location_number;
                }
                i += 1;
                done += 1;
            }
            lowest_seed_number
        }));

        i += 1;
    }

    let mut nums = vec![];
    let mut lowest = 9999999999;
    for t in threads {
        let r = t.join().unwrap();
        if r < lowest {
            lowest = r;
        }
        nums.push(r);
    }

    println!("Nums: {:?}", nums);
    lowest
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
        let expected = 46;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 11, part 2 result: {}", result);
    Ok(())
}

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn solve(lines: Vec<String>) -> i64 {
    let mut galaxies: Vec<Point> = vec![];
    let max_x = (lines.first().unwrap().len() - 1) as i64;
    let max_y = (lines.len() - 1) as i64;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Point::new(x as i64, y as i64))
            }
        }
    }

    let mut expanded_galaxies = galaxies.clone();
    for x in 0..=max_x {
        if galaxies.iter().all(|g| g.x != x) {
            for (i, galaxy) in galaxies.iter().enumerate() {
                if galaxy.x > x {
                    expanded_galaxies[i].x += 1000000 - 1;
                }
            }
        }
    }

    for y in 0..=max_y {
        if galaxies.iter().all(|g| g.y != y) {
            for (i, galaxy) in galaxies.iter().enumerate() {
                if galaxy.y > y {
                    expanded_galaxies[i].y += 1000000 - 1;
                }
            }
        }
    }

    let mut pairs: Vec<(Point, Point)> = vec![];
    for (i, g1) in expanded_galaxies.iter().enumerate() {
        for i2 in (i + 1)..expanded_galaxies.len() {
            let g2 = expanded_galaxies.get(i2).unwrap();
            pairs.push((g1.clone(), g2.clone()));
        }
    }

    let mut result = 0;

    for (g1, g2) in pairs {
        result += (g1.x - g2.x).abs();
        result += (g1.y - g2.y).abs();
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

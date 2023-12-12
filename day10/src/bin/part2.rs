use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 10, part 2 result: {}", result);
    Ok(())
}

fn pipe_from_char(value: char) -> Option<Pipe> {
    if value == '.' {
        None
    } else {
        Some(value.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Pipe {
    Starting,
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            'S' => Pipe::Starting,
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthToEast,
            'J' => Pipe::NorthToWest,
            '7' => Pipe::SouthToWest,
            'F' => Pipe::SouthToEast,
            _ => unreachable!("unrecognized pipe '{}'", value),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn gen_possible_points(point: &Point, max_x: i64, max_y: i64) -> Vec<Point> {
    let mut results: Vec<Point> = vec![];
    for x in [-1, 1] {
        let xx = point.x + x;
        if xx >= 0 && xx <= max_x {
            results.push(Point { x: xx, y: point.y });
        }
    }

    for y in [-1, 1] {
        let yy = point.y + y;
        if yy >= 0 && yy <= max_y {
            results.push(Point { x: point.x, y: yy });
        }
    }
    results
}

impl Pipe {
    fn connects_to(&self, point: &Point, other_point: &Point) -> bool {
        match self {
            Pipe::Starting => true,
            Pipe::Vertical => (other_point.y - point.y).abs() == 1 && point.x == other_point.x,
            Pipe::Horizontal => (other_point.x - point.x).abs() == 1 && point.y == other_point.y,
            Pipe::NorthToEast => {
                (point.y - other_point.y == 1 && point.x == other_point.x)
                    || (point.y == other_point.y && other_point.x - point.x == 1)
            }
            Pipe::NorthToWest => {
                (point.y - other_point.y == 1 && point.x == other_point.x)
                    || (point.y == other_point.y && other_point.x - point.x == -1)
            }
            Pipe::SouthToWest => {
                (point.y - other_point.y == -1 && point.x == other_point.x)
                    || (point.y == other_point.y && other_point.x - point.x == -1)
            }
            Pipe::SouthToEast => {
                (point.y - other_point.y == -1 && point.x == other_point.x)
                    || (point.y == other_point.y && other_point.x - point.x == 1)
            }
        }
    }
}

fn valid_next_piece(last_point: &Point, last_pipe: &Pipe, pipe: &Pipe, point: &Point) -> bool {
    last_pipe.connects_to(last_point, point) && pipe.connects_to(point, last_point)
}

#[derive(Clone, Debug)]
struct PipePoint {
    pipe: Pipe,
    point: Point,
}

fn solve(lines: Vec<String>) -> f64 {
    let mut map: Vec<Vec<Option<Pipe>>> = vec![];
    let mut starting_pos = Point { x: 0, y: 0 };

    for (y, line) in lines.iter().enumerate() {
        let pipes = line
            .chars()
            .map(|c| pipe_from_char(c))
            .collect::<Vec<Option<Pipe>>>();
        for (x, pipe) in pipes.iter().enumerate() {
            if let Some(Pipe::Starting) = pipe {
                starting_pos = Point {
                    x: x as i64,
                    y: y as i64,
                };
            }
        }
        map.push(pipes);
    }

    let max_x = map.first().unwrap().len() - 1;
    let max_y = map.len() - 1;

    let mut paths: Vec<Vec<PipePoint>> = vec![vec![PipePoint {
        point: starting_pos.clone(),
        pipe: Pipe::Starting,
    }]];
    let mut searching = true;
    let mut main_loop: Vec<PipePoint> = vec![];

    while searching {
        let mut next_paths: Vec<Vec<PipePoint>> = vec![];

        if paths.len() == 0 {
            panic!("no more paths to search, failure");
        }

        for path in &paths {
            let last_pipe_point = path.last().unwrap();
            let possibles = gen_possible_points(&last_pipe_point.point, max_x as i64, max_y as i64);

            for possible in possibles {
                if possible != starting_pos && path.iter().any(|p| p.point == possible) {
                    continue;
                }
                let pipe = map
                    .get(possible.y as usize)
                    .unwrap()
                    .get(possible.x as usize)
                    .unwrap();

                if let Some(next_pipe) = pipe {
                    if valid_next_piece(
                        &last_pipe_point.point,
                        &last_pipe_point.pipe,
                        next_pipe,
                        &possible,
                    ) {
                        if next_pipe == &Pipe::Starting {
                            if path.len() > 2 {
                                searching = false;
                                main_loop = path.clone();
                            }
                        } else {
                            let mut new_path = path.clone();
                            new_path.push(PipePoint {
                                point: possible,
                                pipe: next_pipe.clone(),
                            });
                            next_paths.push(new_path);
                        }
                    }
                }
            }
        }

        paths = next_paths;
    }

    let loop_size = main_loop.len();
    let mut vertices: Vec<Point> = vec![];

    for pp in main_loop {
        match pp.pipe {
            Pipe::Vertical => continue,
            Pipe::Horizontal => continue,
            _ => (),
        }
        vertices.push(pp.point.clone());
    }

    let mut area = 0.0;
    for (i, vertex) in vertices.iter().enumerate() {
        let next_index = (i + 1) % vertices.len();
        let next_vertex = vertices.get(next_index).unwrap();
        area += ((vertex.x * next_vertex.y) - (vertex.y * next_vertex.x)) as f64;
    }
    area = area.abs() / 2.0;
    area - (loop_size / 2) as f64 + 1.0
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
        let expected = 4.0;
        let actual = solve(lines("src/bin/sample3.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve4() {
        let expected = 8.0;
        let actual = solve(lines("src/bin/sample4.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve5() {
        let expected = 10.0;
        let actual = solve(lines("src/bin/sample5.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}

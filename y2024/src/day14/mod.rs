use ahash::AHashMap;

#[inline]
pub fn part1(input: &str) -> u32 {
    let mut robots = parse(input);
    let (rows, columns) = match robots.len() {
        12 => (7, 11),
        500 => (103, 101),
        _ => unreachable!("Unsupported grid"),
    };

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.x = if robot.x + robot.v_x < 0 {
                robot.x + robot.v_x + rows
            } else {
                (robot.x + robot.v_x) % rows
            };
            robot.y = if robot.y + robot.v_y < 0 {
                robot.y + robot.v_y + columns
            } else {
                (robot.y + robot.v_y) % columns
            };
        }
    }
    let locations = get_robot_locations(&robots);

    let mut scores: [u32; 5] = [0; 5];
    for (location, robots) in locations {
        let quadrant = if location.0 < rows/2 && location.1 < columns/2 {
            0
        } else if location.0 > rows/2 && location.1 < columns/2 {
            1
        } else if location.0 < rows/2 && location.1 > columns/2 {
            2
        } else if location.0 > rows/2 && location.1 > columns/2 {
            3
        } else {
            4
        };
        scores[quadrant] += robots;
    }

    scores[0] * scores[1] * scores[2] * scores[3]
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let mut robots = parse(input);
    if robots.len() != 500 {
        return 0;
    }
    let (rows, columns) =  (103, 101);

    for i in 0.. {
        let locations = get_robot_locations(&robots);
        if !locations.values().any(|value| *value > 1) {
            return i;
        }
        for robot in robots.iter_mut() {
            robot.x = if robot.x + robot.v_x < 0 {
                robot.x + robot.v_x + rows
            } else {
                (robot.x + robot.v_x) % rows
            };
            robot.y = if robot.y + robot.v_y < 0 {
                robot.y + robot.v_y + columns
            } else {
                (robot.y + robot.v_y) % columns
            };
        }
    }
    
    unreachable!("Loop will terminate, have faith");
}

fn get_robot_locations(robots: &[Robot]) -> AHashMap<(i32, i32), u32> {
    let mut locations = AHashMap::with_capacity(robots.len());
    for robot in robots {
        *locations.entry((robot.x, robot.y)).or_insert(0) += 1;
    }
    locations
}

#[allow(dead_code)]
fn print_grid(robots: &[Robot], rows: i32, columns: i32, iterations: usize) {
    let locations = get_robot_locations(&robots);
    println!("{iterations}");
    for i in 0 .. rows {
        for j in 0 .. columns {
            if locations.contains_key(&(i, j)) {
                print!("{}", locations.get(&(i, j)).unwrap());
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

pub fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .flat_map(|line| scan_fmt::scan_fmt!(line, "p={d},{d} v={d},{d}", i32, i32, i32, i32))
        .map(|(y, x, v_y, v_x)| Robot { x, y, v_x, v_y })
        .collect()
}

common::aoctest!(12, 229632480, 0, 7051);

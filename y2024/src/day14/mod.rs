use ahash::AHashMap;

#[inline]
pub fn part1(input: &str) -> u32 {
    let robots = parse(input);
    let (rows, columns) = match robots.len() {
        12 => (7, 11),
        500 => (103, 101),
        _ => unreachable!("Unsupported grid"),
    };

    let mut scores = [0; 4];
    for robot in robots {
        let final_x = (robot.x + 100 * robot.v_x + 100 * rows) % rows;
        let final_y = (robot.y + 100 * robot.v_y + 100 * columns) % columns;

        let top = final_x < rows / 2;
        let bottom = final_x > rows / 2;
        let left = final_y < columns / 2;
        let right = final_y > columns / 2;

        if top && left {
            scores[0] += 1;
        } else if top && right {
            scores[1] += 1;
        } else if bottom && left {
            scores[2] += 1;
        } else if bottom && right {
            scores[3] += 1;
        }
    }

    scores[0] * scores[1] * scores[2] * scores[3]
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let mut robots = parse(input);
    if robots.len() != 500 {
        return 0;
    }
    let (rows, columns) = (103, 101);
    let mut x_positions = vec![0; robots.len()];
    let mut y_positions = vec![0; robots.len()];

    // Horizontal pattern repeats after every row iterations
    // Vertical pattern repeats after every column iterations
    // Therefore run this simulation only max(rows, columns) times
    let num_iterations = usize::max(rows as usize, columns as usize);
    let mut x_variance = Vec::with_capacity(num_iterations);
    let mut y_variance = Vec::with_capacity(num_iterations);
    for _ in 0..num_iterations {
        for (i, robot) in robots.iter_mut().enumerate() {
            robot.x = (robot.x + robot.v_x + rows) % rows;
            robot.y = (robot.y + robot.v_y + columns) % columns;
            x_positions[i] = robot.x;
            y_positions[i] = robot.y;
        }
        x_variance.push(get_variance(&x_positions));
        y_variance.push(get_variance(&y_positions));
    }

    // Find the index of x_variance and y_variance with minimum value
    let min_x_variance = x_variance
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap()
        .0 as i32
        + 1;
    let min_y_variance = y_variance
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap()
        .0 as i32
        + 1;

    // Uncomment this to print the grid
    // let mut robots = parse(input);
    // for robot in robots.iter_mut() {
    //     robot.x = (robot.x + min_x_variance*robot.v_x + min_x_variance*rows) % rows;
    //     robot.y = (robot.y + min_y_variance*robot.v_y + min_y_variance*columns) % columns;
    // }
    // print_grid(&robots, rows, columns, 0);

    // We need to calculate n - the number of iterations.
    // We also know that n % rows = min_x_variance and n % columns = min_y_variance
    // We can use Chinese Remainder Theorem to find the value of n
    chinese_remainder_theorem(min_x_variance, rows, min_y_variance, columns).unwrap()
}

// This function doesn't bother with the division by n and sqrt because we're only using it for sorting
fn get_variance(positions: &[i32]) -> f64 {
    let n = positions.len() as f64;
    let mean = positions.iter().sum::<i32>() as f64 / n;
    positions
        .iter()
        .map(|&x| (x as f64 - mean).powi(2))
        .sum::<f64>()
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        return (b, 0, 1);
    }

    let (gcd, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let (gcd, x, _) = extended_gcd(a, m);

    if gcd != 1 {
        // Modular multiplicative inverse doesn't exist
        return None;
    }

    // Make sure the result is positive
    Some((x % m + m) % m)
}

fn chinese_remainder_theorem(a1: i32, n1: i32, a2: i32, n2: i32) -> Option<i32> {
    // Check if moduli are coprime
    let (gcd, _, _) = extended_gcd(n1, n2);
    if gcd != 1 {
        return None;
    }

    let n = n1 * n2;

    // Calculate N1 and N2
    let n1_part = n2;
    let n2_part = n1;

    // Calculate x1 and x2 (modular multiplicative inverses)
    let x1 = mod_inverse(n1_part, n1)?;
    let x2 = mod_inverse(n2_part, n2)?;

    // Calculate result using CRT formula
    let mut result = (a1 * n1_part * x1 + a2 * n2_part * x2) % n;

    // Make sure the result is positive
    if result < 0 {
        result += n;
    }

    Some(result)
}

#[allow(dead_code)]
fn print_grid(robots: &[Robot], rows: i32, columns: i32) {
    let locations = get_robot_locations(robots);
    for i in 0..rows {
        for j in 0..columns {
            if locations.contains_key(&(i, j)) {
                print!("{}", locations.get(&(i, j)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_robot_locations(robots: &[Robot]) -> AHashMap<(i32, i32), u32> {
    let mut locations = AHashMap::with_capacity(robots.len());
    for robot in robots {
        *locations.entry((robot.x, robot.y)).or_insert(0) += 1;
    }
    locations
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
        .flat_map(|mut line| parse_robot(&mut line))
        .collect()
}

use winnow::ascii::dec_int;
use winnow::combinator::seq;
use winnow::token::literal;
use winnow::{PResult, Parser};

fn parse_robot(input: &mut &str) -> PResult<Robot> {
    // "p=9,5 v=-3,-3"
    seq! {
        Robot {
            _: literal("p="),
            y: dec_int,
            _: literal(","),
            x: dec_int,
            _: literal(" v="),
            v_y: dec_int,
            _: literal(","),
            v_x: dec_int,
        }
    }
    .parse_next(input)
}

common::aoctest!(12, 229632480, 0, 7051);

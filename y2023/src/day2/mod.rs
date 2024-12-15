#[derive(Debug)]
enum Ball {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Iteration(Vec<Ball>);

#[inline]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(parse_game)
        .filter_map(|(game_number, iterations)| {
            for it in iterations {
                for ball in it.0 {
                    match ball {
                        Ball::Red(n) => {
                            if n > 12 {
                                return None;
                            }
                        }
                        Ball::Green(n) => {
                            if n > 13 {
                                return None;
                            }
                        }
                        Ball::Blue(n) => {
                            if n > 14 {
                                return None;
                            }
                        }
                    }
                }
            }
            Some(game_number)
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter_map(parse_game)
        .map(|(_, iterations)| {
            let (mut min_green, mut min_blue, mut min_red) = (0, 0, 0);
            for ball in iterations.into_iter().flat_map(|iteration| iteration.0) {
                match ball {
                    Ball::Red(n) => {
                        min_red = n.max(min_red);
                    }
                    Ball::Green(n) => {
                        min_green = n.max(min_green);
                    }
                    Ball::Blue(n) => {
                        min_blue = n.max(min_blue);
                    }
                }
            }
            min_green * min_blue * min_red
        })
        .sum()
}

fn parse_game(line: &str) -> Option<(u32, Vec<Iteration>)> {
    let mut iterations = Vec::with_capacity(20);
    let (game, mut remaining) = line.split_once(":")?;
    let game_number = game[5..].parse::<u32>().ok()?;
    while !remaining.is_empty() {
        let (iteration, rem) = parse_iteration(remaining)?;
        remaining = rem;
        iterations.push(iteration);
    }
    Some((game_number, iterations))
}

fn parse_iteration(input: &str) -> Option<(Iteration, &str)> {
    if input.is_empty() {
        return None;
    }
    let mut balls = Vec::with_capacity(7);
    let mut remaining = &input[1..]; // Starts with an empty character ' '
    loop {
        if remaining.is_empty() {
            return Some((Iteration(balls), ""));
        }
        let next_char = remaining.chars().next()?;
        if next_char == ';' {
            return Some((Iteration(balls), &remaining[1..]));
        }
        if next_char == ',' {
            remaining = &remaining[2..];
        }
        let (ball, rem) = parse_ball(remaining)?;
        balls.push(ball);
        remaining = rem;
    }
}

fn parse_ball(input: &str) -> Option<(Ball, &str)> {
    let (number, remaining) = input.split_once(" ")?;
    let number = number.parse::<u32>().ok()?;
    if let Some(stripped) = remaining.strip_prefix("red") {
        return Some((Ball::Red(number), stripped));
    }
    if let Some(stripped) = remaining.strip_prefix("blue") {
        return Some((Ball::Blue(number), stripped));
    }
    if let Some(stripped) = remaining.strip_prefix("green") {
        return Some((Ball::Green(number), stripped));
    }
    None
}

common::aoctest!(8, 2476, 2286, 54911);

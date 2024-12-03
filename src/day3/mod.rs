use regex::Regex;

#[inline]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?P<op1>[0-9]+),(?P<op2>[0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|capture| {
            (
                capture["op1"].parse::<i32>().unwrap(),
                capture["op2"].parse::<i32>().unwrap(),
            )
        })
        .map(|(op1, op2)| op1 * op2)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"((don't)|(do)|mul\((?P<op1>[0-9]+),(?P<op2>[0-9]+)\))").unwrap();
    let mut enabled = true;
    let mut result = 0;
    for capture in re.captures_iter(input) {
        match capture.get(0).map_or("", |m| m.as_str()) {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ => {
                if enabled {
                    let mul = capture["op1"].parse::<i32>().unwrap()
                        * capture["op2"].parse::<i32>().unwrap();
                    result += mul;
                }
            }
        }
    }
    result
}

crate::aoctest!(161, 183380722, 48, 82733683);

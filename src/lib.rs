pub mod computer;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod grid;

#[macro_export]
macro_rules! aoctest {
    ($op_1_test: expr, $op_1: expr, $op_2_test: expr, $op_2: expr) => {
        #[cfg(test)]
        mod tests {
            static TEST_INPUT: &str = include_str!("test-input.txt");
            static FULL_INPUT: &str = include_str!("input.txt");

            #[test]
            fn part_1_test() {
                let output = super::part1(TEST_INPUT);
                assert_eq!(output, $op_1_test);
            }

            #[test]
            fn part_1() {
                let output = super::part1(FULL_INPUT);
                assert_eq!(output, $op_1);
            }

            #[test]
            fn part_2_test() {
                let output = super::part2(TEST_INPUT);
                assert_eq!(output, $op_2_test);
            }

            #[test]
            fn part_2() {
                let output = super::part2(FULL_INPUT);
                assert_eq!(output, $op_2);
            }
        }
    };
}

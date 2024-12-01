pub mod day01;

#[macro_export]
macro_rules! aoctest {
    ($op_1_test: expr, $op_1: expr, $op_2_test: expr, $op_2: expr) => {
        #[cfg(test)]
        mod tests {
            static TEST_INPUT: &str = include_str!("test-input.txt");
            static FULL_INPUT: &str = include_str!("input.txt");

            #[test]
            fn part_1_test() {
                let output = super::part_1(TEST_INPUT);
                assert_eq!(output, $op_1_test);
            }

            #[test]
            fn part_1() {
                let output = super::part_1(FULL_INPUT);
                assert_eq!(output, $op_1);
            }

            #[test]
            fn part_2_test() {
                let output = super::part_2(TEST_INPUT);
                assert_eq!(output, $op_2_test);
            }

            #[test]
            fn part_2() {
                let output = super::part_2(FULL_INPUT);
                assert_eq!(output, $op_2);
            }
        }
    };
}

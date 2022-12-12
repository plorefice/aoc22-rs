pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

#[macro_export]
macro_rules! solutions {
    ($($part:ident => { $code:expr, $exp:expr }),*) => {
        #[cfg(test)]
        mod solutions {
            use super::*;
            $(
                #[test]
                fn $part() {
                    assert_eq!($code, $exp);
                }
            )*
        }
    };
}

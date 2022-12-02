pub mod day01;
pub mod day02;

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

use insta;

#[macro_export]
macro_rules! assert_x86 {
    ($input:expr) => {
        insta::assert_debug_snapshot!(assembly_string($input).unwrap());
    };
}

#[macro_export]
macro_rules! lex_err {
    ($src:expr, $expected:expr) => {
        if let Err(actual) = lex($src) {
            assert_eq!($expected, actual.to_string());
        } else {
            panic!(
                "Expected input '{}' to fail with error '{}'",
                $src, $expected
            );
        }
    };
}

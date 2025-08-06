mod common;

mod valid {
    use crate::assert_x86;
    use cli::codegen;

    /// Since all these tests are essentially the same program, they just return 0, we can validate
    /// that they end up being represented exactly the same as x86 ast.
    #[test]
    fn same_representation() {
        let mut programs = Vec::new();

        let return_0 = codegen(RETURN_0).unwrap();
        programs.push(codegen(NO_NEWLINES).unwrap());
        programs.push(codegen(NEWLINES).unwrap());
        programs.push(codegen(SPACES).unwrap());
        programs.push(codegen(TABS).unwrap());

        for p in programs {
            assert_eq!(return_0, p);
        }
    }

    const RETURN_0: &str = "int main(void) {
        return 0;
    }";

    #[test]
    fn return_0() {
        assert_x86!(RETURN_0);
    }

    const RETURN_2: &str = "int main(void) {
        return 2;
    }";

    #[test]
    fn return_2() {
        assert_x86!(RETURN_2);
    }

    const NO_NEWLINES: &str = "int main(void){return 0;}";

    #[test]
    fn no_newlines() {
        assert_x86!(NO_NEWLINES);
    }

    const MULTI_DIGITS: &str = "int main(void) {
        return 100;
    }";

    #[test]
    fn multi_digit() {
        assert_x86!(MULTI_DIGITS);
    }

    const NEWLINES: &str = "int
            main
            (
            void
            )
            {
            return
            0
            ;
        }";

    #[test]
    fn newlines() {
        assert_x86!(NEWLINES);
    }

    const SPACES: &str = " int   main    (  void)  {   return  0 ; } ";

    #[test]
    fn spaces() {
        assert_x86!(SPACES);
    }

    const TABS: &str = " int	main	(	void)	{	return	0	;	} ";

    #[test]
    fn tabs() {
        assert_x86!(TABS);
    }
}

mod invalid_lex {
    use crate::lex_err;
    use cli::lex;

    #[test]
    fn at_sign() {
        let src = "int main(void) { return 0@1; }";

        lex_err!(src, "Invalid char '@'");
    }

    #[test]
    fn backslash() {
        let src = "\\";

        lex_err!(src, "Invalid char '\\'");
    }

    #[test]
    fn backtick() {
        let src = "`";

        lex_err!(src, "Invalid char '`'");
    }

    #[test]
    fn invalid_identifier() {
        let src = "int main(void) { return 1foo; }";

        lex_err!(src, "Invalid char in numeric constant 'f'");
    }

    #[test]
    fn invalid_identifier_2() {
        let src = "int main(void) { return @b; }";

        lex_err!(src, "Invalid char '@'");
    }
}

mod invalid_parse {
    use crate::assert_x86;
    use cli::codegen;
}

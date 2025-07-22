use ast::{Constant, ConstantId, ExprId, NodeKind, Stmt, StmtId, Tree};
use lex::{Token, TokenType, TokenizedOutput, token::Keyword};

pub struct Parser<'src> {
    input: TokenizedOutput<'src>,
    nodes: Tree,
    cur_token: usize,
}

impl<'src> Parser<'src> {
    pub fn from_tokens(tokens: TokenizedOutput) -> Parser {
        // Size optimization where we "guess" we'll have around the same number of ast nodes and tokens
        // This is not exactly correct, but it's good enough
        let len = tokens.len();
        Parser {
            input: tokens,
            nodes: Tree::with_capacity(len),
            cur_token: 0,
        }
    }

    fn expect(&mut self, ttype: TokenType) -> Token {
        let token = self.input.get(self.cur_token).unwrap();

        assert_eq!(token.ttype, ttype);
        self.cur_token += 1;

        token
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Token {
        let token = self.expect(TokenType::Ident);

        let source = self.input.token_text(token.handle);

        // TODO propagate error
        assert_eq!(source, format!("{}", keyword));

        token
    }

    /// <statement> ::= "return" <expr> ";"
    fn parse_statement(&mut self) -> StmtId {
        let return_keyword = self.expect_keyword(Keyword::Return);

        let return_expr = self.parse_expr();

        self.expect(TokenType::Semicolon);

        let stmt = Stmt::Return {
            expr: return_expr,
            token: return_keyword,
        };

        self.nodes.push(stmt)
    }

    /// <expr> ::= <constant>
    fn parse_expr(&mut self) -> ExprId {
        todo!()
    }

    /// <constant> = <int>
    fn parse_constant(&mut self) -> ConstantId {
        let token = self.expect(TokenType::Constant);
        let token_source = self.input.token_text(token.handle);
        println!("{}", token_source);
        let value: i64 = token_source.parse().unwrap();

        let constant = Constant { value, token };

        self.nodes.push(constant)
    }
}

#[cfg(test)]
mod tests {
    use lex::{Lexer, TokenizedOutput};

    use crate::Parser;

    fn lex<'src>(source: &'src str) -> Parser<'src> {
        let tokens = Lexer::lex(source);
        let parser = Parser::from_tokens(tokens);
        parser
    }

    #[test]
    fn parse_zero() {
        let source = "0";

        let mut parser = lex(source);

        let constant_id = parser.parse_constant();
        assert_eq!(usize::from(constant_id), 0);

        let constant = parser.nodes[constant_id];
        assert_eq!(constant.value, 0);
        assert_eq!(constant.token, parser.input.get(0).unwrap());
    }

    #[test]
    fn parse_one() {
        let source = "1";

        let mut parser = lex(source);

        let constant_id = parser.parse_constant();
        assert_eq!(usize::from(constant_id), 0);

        let constant = parser.nodes[constant_id];
        assert_eq!(constant.value, 1);
        assert_eq!(constant.token, parser.input.get(0).unwrap());
    }

    #[test]
    fn parse_i64_max() {
        let source = format!("{}", i64::MAX);

        let mut parser = lex(&source);

        let constant_id = parser.parse_constant();
        assert_eq!(usize::from(constant_id), 0);

        let constant = parser.nodes[constant_id];
        assert_eq!(constant.value, i64::MAX);
        assert_eq!(constant.token, parser.input.get(0).unwrap());
    }

    // FIXME: These tests exercise negative number parsing which we don't handle just yet
    // #[test]
    // fn parse_minus_one() {
    //     let source = format!("{}", -1);
    //
    //     let mut parser = lex(&source);
    //
    //     let constant_id = parser.parse_constant();
    //     assert_eq!(usize::from(constant_id), 0);
    //
    //     let constant = parser.nodes[constant_id];
    //     assert_eq!(constant.value, -1);
    //     assert_eq!(constant.token, parser.input.get(0).unwrap());
    // }
    //
    // #[test]
    // fn parse_i64_min() {
    //     let source = format!("{}", i64::MIN);
    //
    //     let mut parser = lex(&source);
    //
    //     let constant_id = parser.parse_constant();
    //     assert_eq!(usize::from(constant_id), 0);
    //
    //     let constant = parser.nodes[constant_id];
    //     assert_eq!(constant.value, i64::MIN);
    //     assert_eq!(constant.token, parser.input.get(0).unwrap());
    // }
}

use ast::{
    Constant, ConstantId, Expr, ExprId, FnDef, FnDefId, Ident, IdentId, Program, ProgramId, Stmt,
    StmtId, Tree,
};
use lex::{Token, TokenType, TokenizedOutput, token::Keyword};
use tracing::{Level, span};

pub struct Parser<'src> {
    input: &'src TokenizedOutput<'src>,
    nodes: Tree<'src>,
    cur_token: usize,
}

impl<'src> Parser<'src> {
    pub fn from_tokens(tokens: &'src TokenizedOutput<'src>) -> Parser<'src> {
        // Size optimization where we "guess" we'll have around the same number of ast nodes and tokens
        // This is not exactly correct, but it's good enough
        let len = tokens.len();
        Parser {
            input: tokens,
            nodes: Tree::with_capacity(tokens, len),
            cur_token: 0,
        }
    }

    pub fn nodes(&'src self) -> &'src Tree<'src> {
        &self.nodes
    }

    fn expect(&mut self, ttype: TokenType) -> Token {
        let token = self.input.get(self.cur_token).unwrap();

        assert_eq!(token.ttype, ttype);
        self.cur_token += 1;

        token
    }

    fn expect_ident(&mut self) -> IdentId {
        let token = self.expect(TokenType::Ident);

        let ident = Ident { token };

        self.nodes.push(ident)
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> Token {
        let token = self.expect(TokenType::Ident);

        let source = self.input.token_text(token.handle);

        // TODO propagate error
        assert_eq!(source, format!("{}", keyword));

        token
    }

    pub fn parse(&mut self) {
        let _ = span!(Level::TRACE, "Parsing").entered();

        let main = self.parse_function_def();
        let program_node = Program { main };
        self.nodes.push::<Program, ProgramId>(program_node);
    }

    fn parse_function_def(&mut self) -> FnDefId {
        let _type_specifier = self.expect_keyword(Keyword::Int);

        let function_name = self.expect_ident();

        self.expect(TokenType::OpenParen);
        self.expect_keyword(Keyword::Void);
        self.expect(TokenType::CloseParen);
        self.expect(TokenType::OpenBrace);

        let stmt = self.parse_statement();

        self.expect(TokenType::CloseBrace);

        let fn_def = FnDef {
            name: function_name,
            body: stmt,
        };

        self.nodes.push(fn_def)
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
        let value = self.parse_constant();
        let expr = Expr::Constant { constant: value };
        self.nodes.push(expr)
    }

    /// <constant> = <int>
    fn parse_constant(&mut self) -> ConstantId {
        let token = self.expect(TokenType::Constant);
        let token_source = self.input.token_text(token.handle);
        let value: i64 = token_source.parse().unwrap();

        let constant = Constant { value, token };

        self.nodes.push(constant)
    }
}

#[cfg(test)]
mod tests {
    use lex::{Lexer, TokenizedOutput};

    use crate::Parser;

    #[test]
    fn parse_zero() {
        let source = "0";

        let tokens = Lexer::lex(source);
        let mut parser = Parser::from_tokens(&tokens);

        let constant_id = parser.parse_constant();
        assert_eq!(usize::from(constant_id), 0);

        let constant = parser.nodes[constant_id];
        assert_eq!(constant.value, 0);
        assert_eq!(constant.token, parser.input.get(0).unwrap());
    }

    #[test]
    fn parse_one() {
        let source = "1";

        let tokens = Lexer::lex(source);
        let mut parser = Parser::from_tokens(&tokens);

        let constant_id = parser.parse_constant();
        assert_eq!(usize::from(constant_id), 0);

        let constant = parser.nodes[constant_id];
        assert_eq!(constant.value, 1);
        assert_eq!(constant.token, parser.input.get(0).unwrap());
    }

    #[test]
    fn parse_i64_max() {
        let source = format!("{}", i64::MAX);

        let tokens = Lexer::lex(&source);
        let mut parser = Parser::from_tokens(&tokens);

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

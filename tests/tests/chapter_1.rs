use lex::Lexer;

#[test]
fn multi_digit() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/multi_digit.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}
#[test]
fn no_newlines() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/no_newlines.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}
#[test]
fn return_0() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/return_0.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}
#[test]
fn tabs() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/tabs.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}
#[test]
fn newlines() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/newlines.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}
#[test]
fn spaces() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/spaces.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}
#[test]
fn return_2() {
let source = std::fs::read_to_string("../examples/chapter_1/valid/return_2.c").unwrap();
let tokens = Lexer::lex(&source);
insta::assert_debug_snapshot!(tokens);
}

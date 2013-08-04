use lexer::{Token, TokenType, Lexer, COMMAND, SEMICOLON, EOF};
use std::libc::exit;

pub struct Parser {
    input: Lexer,
    k: uint,
    index: uint,
    lookahead: ~[Token],
}

impl Parser {
    pub fn consume(&mut self) {
        self.lookahead[self.index] = self.input.next_token();
        self.index = (self.index + 1) % self.k;
    }

    pub fn LT(&mut self, i: uint) -> Token {
        let token = copy self.lookahead[(self.index + i - 1) % self.k];
        token
    }

    pub fn LA(&mut self, i: uint) -> TokenType {
        self.LT(i).ttype
    }

    pub fn tmatch(&mut self, ttype: TokenType) {
        let current_ttype = self.LA(0) as int;
        let ttype = ttype as int;
        if (current_ttype == ttype) {
            self.consume();
        } else {
            error!("Expecting: " + ttype.to_str() + " Found: " + current_ttype.to_str());
            unsafe {
                exit(1);
            }
        }
    }

    pub fn input(&mut self) {
        self.commands();
        self.tmatch(EOF);
    }

    pub fn commands(&mut self) {
        self.command();
        while (self.LA(0) as int == SEMICOLON as int) {
            self.tmatch(SEMICOLON);
            self.command();
        }
    }

    pub fn command(&mut self) {
        self.tmatch(COMMAND);
    }
}

#[main]
pub fn main() {
    let mut lex = Lexer { input: ~"foo; bar; baz", index: 0, currentChar: 't', eof: false };
    let mut parser = Parser { input: lex, k: 1, index: 0, lookahead: ~[] };
    parser.lookahead.grow(parser.k, &Token {text: ~" ", ttype: COMMAND});
    let mut k_count = 1;
    while (k_count <= parser.k) {
        k_count += 1;
        parser.consume();
    }
    parser.input();
}

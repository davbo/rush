use lexer::{Token, TokenType, Lexer};

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
        copy self.lookahead[(self.index + i) % self.k]
    }

    pub fn LA(&mut self, i: uint) -> TokenType {
        self.LT(i).ttype
    }

    pub fn tmatch(&mut self, ttype: TokenType) {
        let current_ttype = self.LA(0);
        match ttype {
            current_ttype => {
                self.consume();
            }
        }
    }
}

#[main]
pub fn main() {
    let mut lex = Lexer { input: ~"te;st st2ring|", index: 0, currentChar: 't', eof: false };
    let mut parser = Parser { input: lex, k: 4, index: 0, lookahead: ~[] };
}

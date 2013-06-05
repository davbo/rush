mod lexer {
    extern mod core;


    enum TokenType {
        COMMAND,
        ARGUMENT,
        EXIT,
        PIPE,
        SEMICOLON,
        EOF,
    }

    pub struct Token {
        text: ~str,
        ttype: TokenType,
    }

    impl to_str::ToStr for Token {
        fn to_str(&self) -> ~str {
            copy self.text
        }
    }

    pub struct Lexer {
        input: ~str,
        currentChar: char,
        index: uint,
        eof: bool,
    }

    impl Lexer {

        pub fn consume(&mut self) {
            self.index += 1;
            if (self.index >= str::len(self.input)) {
                self.eof = true;
            } else {
                self.currentChar = str::char_at(self.input, self.index);
            }
        }

        pub fn whitespace(&mut self) {
            while (char::is_whitespace(self.currentChar) && !self.eof) {
                self.consume();
            }
        }

        pub fn command(&mut self) -> Token {
            let start = self.index;
            while (char::is_alphanumeric(self.currentChar) && !self.eof) {
                self.consume();
            }
            Token {text: str::slice(self.input, start, self.index).to_owned(), ttype: COMMAND}
        }

        pub fn next_token(&mut self) -> Token {
            while !self.eof {
                match self.currentChar {
                    ' ' | '\n' | '\r' | '\t' => {
                        self.whitespace();
                        loop;
                    }
                    ';' => {
                        self.consume();
                        return Token {text: ~";", ttype: SEMICOLON};
                    }
                    '|' => {
                        self.consume();
                        return Token {text: ~"|", ttype: PIPE};
                    }
                    _ => {
                        return self.command();
                    }
                }
            }
            return Token {text: ~" ", ttype: EOF};
        }

    }
    pub fn main() {
        let mut lex = Lexer { input: ~"te;st st2ring|", index: 0, currentChar: 't', eof: false };
        let mut t = lex.next_token();
        loop {
            match t.ttype {
                EOF => return,
                _   => {
                    io::println(t.to_str());
                    t = lex.next_token();
                }
            }

        }
    }
}

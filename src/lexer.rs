mod lexer {
    extern mod core;

    enum TokenType {
        COMMAND,
        EXIT,
        PIPE,
        EOF,
    }

    pub struct Token {
        text: ~str,
        ttype: TokenType,
    }

    pub struct Lexer {
        input: ~str,
        mut currentChar: char,
        mut index: int,
    }
}

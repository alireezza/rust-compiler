pub enum TokenKind { 
    Number(i64),
    Plus, 
    Minus, 
    AsterisR, 
    Slash, 
    LeftParen, 
    RightParen, 


}

pub struct TextSpan { 
    start: usize, 
    end: usize,
    literal: String, 
}


impl TextSpan { 
    pub fn new(start: usize, end: usize, literal: String) -> Self { 
        TextSpan { start, end, literal }
    }
    pub fn length(&self) -> usize { 
        self.end - self.start
    }
}


pub struct Token { 
    kind: TokenKind,
    span: TextSpan,

}

impl Token { 
    pub fn new(kine: TokenKind, span: TextSpan) -> Self { 
        Token { kind, span }
    }
}

pub struct Lexer<'a> { 
    input: std::str::Chars<'a>,
    pub fn new(input: &'a str) -> Self { 
        Self { input: input.chars().peekable() } 

    }
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
}

pub struct Token { 
    kind: TokenKind,

}
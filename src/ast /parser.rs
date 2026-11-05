pub struct Parser { 

    tokens: Vec<super::lexer::Token> 
    current: usize, 
} 

impl Parser { 

    pub fn new() -> Self { 
        Self { tokens: Vec::new(), current: 0 }

    }

    pub fn from_input(input: &str) -> Self { 
        let mut lexer 
    }
}
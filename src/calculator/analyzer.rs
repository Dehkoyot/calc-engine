use std::fmt;
use calculator::operation;
use calculator::operation::Operation::*;

pub struct Analyzer {
    pub curr:  char,
    pub pos: usize,
    pub src: String,
    pub eof: bool
}

impl Analyzer {
    pub fn new(src: &str) -> Analyzer {
        let mut l = Analyzer {
            curr: '\0',
            pos: 0,
            src: src.to_string(),
            eof: false
        };
        if l.pos >= src.len() {
            l.eof = true;
        } else {
            l.curr = src.chars().nth(0).unwrap();
        }
        l
    }
    pub fn next_operation(&mut self) -> Result<operation::Operation, String> {
        if self.eof {
            return Ok(EOF);
        }
        self.consume_whitespace();
        match self.curr {
            '(' => {self.bump(); Ok(LBRACE)}
            ')' => {self.bump(); Ok(RBRACE)}
            c if c.is_digit(10) => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while (self.curr.is_digit(10) || self.curr == '.') && !self.eof{
                    self.bump();
                    end += 1;
                }
                Ok(NUMBER(self.src[start..end].parse::<f64>().unwrap()))
            }
            c if c.is_alphabetic() => {
                let start = self.pos;
                let mut end = start + 1;
                self.bump();
                while self.curr.is_alphabetic() && !self.eof {
                    self.bump();
                    end += 1;
                }
                Ok(SYMBOL(self.src[start..end].to_string()))
            }
            '+' => {self.bump(); Ok(PLUS)}
            '-' => {self.bump(); Ok(MINUS)}
            '*' => {self.bump(); Ok(MULTIPLY)}
            '/' => {self.bump(); Ok(DIVIDE)}
            '^' => {self.bump(); Ok(DEGREE)}
            c => { Err(format!("unexpected operation {} at position {}", c, self.pos)) }
        }
    }
    pub fn bump(&mut self) {
        self.pos += 1;
        if self.pos >= self.src.len() {
            self.eof = true;
            return;
        }
        self.curr = self.src.chars().nth(self.pos).unwrap();
    }

    pub fn consume_whitespace(&mut self) {
        while is_whitespace(self.curr) {
            self.bump();
        }
    }
}
pub fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' => true,
        _ => false
    }
}


impl fmt::Display for Analyzer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.src)
    }
}
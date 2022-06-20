use self::Operation::*;


#[derive(Debug,PartialEq, Clone)]
pub enum Operation {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    DEGREE,
    LBRACE,
    RBRACE,
    NUMBER(f64),
    SYMBOL(String),
    EOF
}

impl Operation {
    pub fn info(&self) -> Option<(usize, usize)> {
        match *self {
            PLUS | MINUS => Some((10, 0)),
            MULTIPLY | DIVIDE => Some((20, 0)),
            DEGREE => Some((30, 1)),
            _ => { None }
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            PLUS => '+',
            MINUS => '-',
            MULTIPLY => '*',
            DIVIDE => '/',
            DEGREE => '^',
            LBRACE => '(',
            RBRACE => ')',
            NUMBER(_) => 'N',
            SYMBOL(_) => 'S',
            EOF => 'E',
        }
    }
}

pub fn is_eof(t: &Operation) -> bool{
    match t {
        &EOF => true,
        _ => false
    }
}
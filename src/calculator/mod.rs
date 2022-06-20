pub use self::operation::Operation::*;

pub mod ev;
pub mod analyzer;
pub mod operation;


pub struct Calculator {
    pub current: operation::Operation,
    pub analyzer: analyzer::Analyzer,
    pub peeked: Option<operation::Operation>,
}

impl Calculator {
    pub fn new(input: &str) -> Calculator {
        let l = analyzer::Analyzer::new(input);
        let p = Calculator {
            current: EOF,
            peeked: None,
            analyzer: l
        };
        p
    }

    pub fn parse(&mut self) -> Result<Box<dyn ev::Evaluation>, String> {
        self.expr(1)
    }

    pub fn expr(&mut self, prec: usize) -> Result<Box<dyn ev::Evaluation>, String> {
        let mut lhs = self.atom()?;
        let mut rhs;
        loop {
            let curr = self.peek_operation()?;
            if operation::is_eof(&curr) {
                break;
            }
            let info_tuple = curr.info();
            if info_tuple.is_none() {
                break;
            }
            let (op_prec, op_assoc) = info_tuple.unwrap();
            if op_prec < prec {
                break;
            }
            self.next_operation()?;
            match op_assoc {
                0 => {
                    rhs = self.expr(op_prec + 1)?;
                }
                _  => {
                    rhs = self.expr(op_prec)?;
                }
            }
            lhs = self.op(curr, lhs, rhs);

        }
        Ok(lhs)
    }

    pub fn atom(&mut self) -> Result<Box<dyn ev::Evaluation>, String> {

        match self.peek_operation()? {
            EOF => { Ok(Box::new( ev::Num {num: 0f64})) }
            LBRACE => {
                self.expect('(')?;
                let e = self.expr(1)?;
                self.expect(')')?;
                Ok(e)
            }
            NUMBER(val) => {
                self.next_operation()?;
                Ok(Box::new( ev::Num { num: val }))
            }
            a => {
                Err(format!("Unrecognized character: {:?}", a))
            }
        }
    }

    pub fn op (&self, op: operation::Operation, lhs: Box<dyn ev::Evaluation>, rhs: Box<dyn ev::Evaluation>)
            -> Box<dyn ev::Evaluation> {
        match op {
            PLUS => {
                Box::new( ev::Add {
                    left: lhs,
                    right: rhs
                })
            }
            MINUS => {
                Box::new( ev::Sub {
                    left: lhs,
                    right: rhs
                })
            }
            MULTIPLY => {
                Box::new( ev::Mul {
                    left: lhs,
                    right: rhs
                })
            }
            DIVIDE => {
                Box::new( ev::Div {
                    left: lhs,
                    right: rhs
                })
            }
            DEGREE => {
                Box::new( ev::Pow {
                    base: lhs,
                    exponent: rhs
                })
            }
            o => {
                panic!("unrecognized op: {:?}", o);
            }
        }
    }
}

impl Calculator {
    pub fn expect(&mut self, tok: char) -> Result<(), String> {
        self.next_operation()?;
        if self.current.to_char() != tok {
            return Err(format!("expected {:?} but found {}", tok, self.current.to_char()));
        }
        Ok(())
    }
    pub fn peek_operation(&mut self) -> Result<operation::Operation, String> {
        if self.peeked.is_none() {
            self.peeked = Some(self.analyzer.next_operation()?);
        }
        Ok(self.peeked.clone().unwrap())
    }
    pub fn next_operation(&mut self) -> Result<(), String> {
        match self.peeked {
            Some(ref mut pk) => {
                self.current = pk.clone();
            }
            None => {
                self.current = self.analyzer.next_operation()?;
            }
        }
        self.peeked = None;
        Ok(())
    }
}
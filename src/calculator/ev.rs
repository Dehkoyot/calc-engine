extern crate std;
use std::collections::HashMap;
use std::f64;

pub trait Evaluation {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64>;
}

pub struct Num {
    pub num: f64
}

impl Evaluation for Num {
    fn eval(&self, _env: &mut HashMap<String, f64>) -> Option<f64> {
        Some(self.num)
    }
}

pub struct Add {
    pub left: Box<dyn Evaluation>,
    pub right: Box<dyn Evaluation>,
}

impl Evaluation for Add {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l + r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Sub {
    pub left: Box<dyn Evaluation>,
    pub right: Box<dyn Evaluation>,
}

impl Evaluation for Sub {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l - r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Mul {
    pub left: Box<dyn Evaluation>,
    pub right: Box<dyn Evaluation>,
}

impl Evaluation for Mul {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l*r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Div {
    pub left: Box<dyn Evaluation>,
    pub right: Box<dyn Evaluation>,
}

impl Evaluation for Div {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l/r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Pow {
    pub base: Box<dyn Evaluation>,
    pub exponent: Box<dyn Evaluation>
}

impl Evaluation for Pow {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.base.eval(env) {
            Some(b) => {
                match self.exponent.eval(env) {
                    Some(e) => Some(b.powf(e)),
                    None => None
                }
            }
            None => None
        }
    }
}
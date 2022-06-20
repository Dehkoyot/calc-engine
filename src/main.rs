use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
pub mod calculator;

fn evaluate(input: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
    let mut p = calculator::Calculator::new(input);
    let ev = p.parse()?;
    match ev.eval(env) {
        Some(result) => Ok(result),
        None => Err("No value for that expression!".to_string())
    }
}

pub fn main() {
    let mut env = HashMap::new();
    let stdin = io::stdin();


    loop {
        print!(">> ");
        io::stdout().flush().ok();

        let mut input = String::new();

        match stdin.read_line(&mut input) {
            Ok(_) => {

                if input.len() == 0 {
                    println!("");
                    return;
                }

                let expression_text = input.trim_end();

                let result = evaluate(expression_text, &mut env);
                match result {
                    Ok(value) => {
                        println!("= {}", value);
                    }
                    Err(s) => {
                        println!("Error: {}", s);
                    }
                }
                io::stdout().flush().ok();
            }
            Err(_) => {
                println!("");
                return;
            }
        }
    }
}
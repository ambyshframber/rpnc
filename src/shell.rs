use fastrand::i64;
use std::collections::HashMap;
use std::f64::consts::*;
use rustyline::{error::ReadlineError, Editor};

use crate::utils::RpnError;

#[derive(Default, Debug)]
pub struct Shell {
    stack: Vec<f64>,

    bonus_words: HashMap<String, Vec<String>>,
    pub interactive: bool,
    in_comment: bool,
    compiling: bool,
    found_name: bool,
    cur_word_name: String,
    cur_word_buf: Vec<String>,
    not_doing_an_if: bool,
    if_layers: i32
}

impl Shell {
    pub fn new() -> Shell {
        Shell::default()
    }
    pub fn get_exit_val(&self) -> i32 {
        if !self.stack.is_empty() {
            self.stack[self.stack.len() - 1] as i32
        }
        else {
            0
        } 
    }
    pub fn run(&mut self) -> Result<(), RpnError> {
        let mut rl = Editor::<()>::new();
        loop {
            let prompt = if self.interactive { "> " } else { "" };
            let line = rl.readline(prompt);
            match line {
                Ok(s) => {
                    if self.do_line(&s) {
                        return Ok(())
                    }
                    rl.add_history_entry(s);
                }
                Err(ReadlineError::Interrupted) => {
                    return Ok(())
                }
                Err(ReadlineError::Eof) => {
                    return Ok(())
                }
                Err(e) => {
                    return Err(e.into())
                }
            }
        }
    }
    pub fn do_line(&mut self, l: &str) -> bool {
        if l.starts_with('#') {
            // posix
            return false
        }
        for w in l.split_ascii_whitespace() {
            match self.do_word(w) {
                Ok(halted) => {
                    if halted {
                        return true
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            }
        }

        false
    }

    fn do_word(&mut self, w: &str) -> Result<bool, RpnError> {
        if self.in_comment {
            if w == ")" {
                self.in_comment = false
            }
            return Ok(false);
        } else if w == "(" {
            self.in_comment = true;
            return Ok(false);
        }
        if self.compiling {
            if w == ";" {
                // end compile
                self.compiling = false;
                self.bonus_words.insert(
                    self.cur_word_name.clone(),
                    self.cur_word_buf.clone()
                );
            } else if self.found_name {
                // name HAS been found
                self.cur_word_buf.push(w.into())
            } else {
                // name HAS NOT been found
                self.found_name = true;
                self.cur_word_name = w.into()
            }
            return Ok(false);
        }
        if self.not_doing_an_if {
            if w == "if" {
                self.if_layers += 1
            }
            else if w == "then" {
                self.if_layers -= 1
            }
            if self.if_layers == -1 {
                self.not_doing_an_if = false
            }
            return Ok(false)
        }
        match w.parse() {
            Ok(v) => self.stack.push(v),
            Err(_) => {
                match w {
                    "+" | "-" | "*" | "/" | "%" => {
                        let ops = self.get_top_n(2)?;
                        let a = ops[0]; // allows rollback on stack underflow
                        let b = ops[1];
                        match w.as_bytes()[0] {
                            b'+' => self.stack.push(a + b),
                            b'-' => self.stack.push(b - a),
                            b'*' => self.stack.push(a * b),
                            b'/' => self.stack.push(b / a),
                            b'%' => self.stack.push(b % a),

                            _ => unreachable!(),
                        }
                    }
                    "**" => {
                        // pow
                        let ops = self.get_top_n(2)?;
                        self.stack.push(ops[1].powf(ops[0]))
                    }
                    "log" => {
                        let ops = self.get_top_n(2)?;
                        self.stack.push(ops[1].log(ops[0]))
                    }
                    "ln" => {
                        let x = self.get_top()?;
                        self.stack.push(x.ln())
                    }
                    "sin" => {
                        let a = self.get_top()?;
                        self.stack.push(a.sin())
                    }
                    "cos" => {
                        let a = self.get_top()?;
                        self.stack.push(a.cos())
                    }
                    "tan" => {
                        let a = self.get_top()?;
                        self.stack.push(a.tan())
                    }
                    "asin" => {
                        let a = self.get_top()?;
                        self.stack.push(a.asin())
                    }
                    "acos" => {
                        let a = self.get_top()?;
                        self.stack.push(a.acos())
                    }
                    "atan" => {
                        let a = self.get_top()?;
                        self.stack.push(a.atan())
                    }
		            "fact" => {
			            let mut a = self.get_top()?;
                        self.stack.push(if a.fract() == 0.0 && a.is_sign_positive() {
                            let mut x = 1.0;
                                while a > 0.0 {
                                    x *= a;
                                    a -= 1.0;
                                    if x == f64::INFINITY {
                                        break
                                    }
                                }
                                x
                            }
                            else {
                                f64::NAN
                            }
                        )
		            }
                    "." => println!("{}", self.peek_top()?),
                    ".s" => {
                        for i in self.stack.iter().rev() {
                            print!("{} ", i)
                        }
                        println!()
                    }
                    ".stdf" => {
                        let mut x = self.peek_top()?;
                        if x.is_sign_negative() {
                            x = x.abs();
                            print!("-")
                        }
                        let exp = x.log(10.0).floor();
                        let disp = x / 10f64.powf(exp);
                        println!("{:.4} * 10^{}", disp, exp)
                    }
                    "swp" => {
                        let ops = self.get_top_n(2)?;
                        for i in ops.iter() {
                            self.stack.push(*i)
                        }
                    }
                    "pop" => {
                        self.get_top()?;
                    }
                    "dup" => {
                        // duplicate top item of stack
                        let i = self.get_top()?;
                        self.stack.push(i);
                        self.stack.push(i);
                    }
                    "over" => {
                        let op = self.peek_top_n(2)?[0];
                        self.stack.push(op)
                    }
                    "rot" => {
                        // a b c -- b c a
                        let ops = self.get_top_n(3)?;
                        self.stack.push(ops[1]);
                        self.stack.push(ops[0]);
                        self.stack.push(ops[2]);
               	    }
                    "-rot" => {
                        // a b c -- c a b
                        let ops = self.get_top_n(3)?;
                        self.stack.push(ops[0]);
                        self.stack.push(ops[2]);
                        self.stack.push(ops[1]);
                    }
                    "pick" => {
                        // x_u ... x_1 x_0 u -- x_u ... x_1 x_0 x_u
                        let u = self.get_top()? as usize;
                        if u >= self.stack.len() {
                            return Err(RpnError::StackUnderflow);
                        }
                        let x_u = self.stack[self.stack.len() - (u + 1)];
                        self.stack.push(x_u)
                    }
                    "put" => {
                        // x_u ... x_1 x_0 y u -- y ... x_1 x_0 x_u
                        let u = self.get_top()? as usize;
                        let y = self.get_top()?; // x_u ... x_1 x_0
                        let l = self.stack.len();
                        if u >= l {
                            return Err(RpnError::StackUnderflow);
                        }
                        self.stack[l - (u + 1)] = y
                    }
                    "pi" => self.stack.push(PI),
                    "e" => self.stack.push(E),
                    "dice" => {
                        let a = self.get_top()? as i64;
                        let x: i64 = i64(0..a);
                        self.stack.push(x as f64)
                    }
                    "if" => {
                        self.not_doing_an_if = self.get_top()? == 0.0
                    }
                    "then" => {}
                    "round" => {
                        let x = self.get_top()?.round();
                        self.stack.push(x)
                    }
                    "clear" => self.stack.clear(),
                    "bye"|"exit"|"quit" => return Ok(true), // cooler than "exit" or "quit"

                    // forth time!
                    ":" => {
                        self.compiling = true;
                        self.found_name = false;
                        self.cur_word_buf.clear()
                    }
                    _ => match self.bonus_words.get(w) {
                        Some(words) => {
                            for w in words.clone() {
                                self.do_word(&w)?;
                            }
                        }
                        None => return Err(RpnError::UndefinedWord(w.into())),
                    },
                }
            }
        }

        Ok(false)
    }

    fn get_top(&mut self) -> Result<f64, RpnError> {
        self.stack.pop().ok_or(RpnError::StackUnderflow)
    }
    fn get_top_n(&mut self, n: usize) -> Result<Vec<f64>, RpnError> {
        let mut ret = Vec::new();
        for _ in 0..n {
            match self.stack.pop() {
                Some(v) => ret.push(v),
                None => {
                    // underflow!
                    for v in ret.iter().rev() {
                        // put items back on stack
                        self.stack.push(*v)
                    }
                    return Err(RpnError::StackUnderflow);
                }
            }
        }
        Ok(ret)
    }

    fn peek_top(&self) -> Result<f64, RpnError> {
        if self.stack.is_empty() {
            Err(RpnError::StackUnderflow)
        } else {
            Ok(self.stack[self.stack.len() - 1])
        }
    }
    fn peek_top_n(&self, n: usize) -> Result<&[f64], RpnError> {
        if self.stack.len() >= n {
            Ok(&self.stack[self.stack.len() - n..])
        } else {
            Err(RpnError::StackUnderflow)
        }
    }
}

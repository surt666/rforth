use anyhow::Result;
use std::io::{self, stdout, Write, BufRead};
use std::{
    collections::{HashMap},
};
use strum_macros::{Display};
use maplit::*;

#[derive(Debug)]
struct Stack<T> {
  stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
	Stack { stack: Vec::new() }
    }
    
    fn pop(&mut self) -> Option<T> {
	self.stack.pop()
    }
    
    fn push(&mut self, item: T) {
	self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
	self.stack.is_empty()
    }

    fn length(&self) -> usize {
	self.stack.len()
    }

    fn peek(&self) -> Option<&T> {
	self.stack.last()
    }
}

// impl fmt::Display for Stack<T> {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self)
//     }
// }

#[derive(Display)]
enum CW {
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Dump,
}

fn dup(ds: &mut Stack<String>) {
    let a = ds.peek().unwrap();
    ds.push(a.to_string())
}

fn swap(ds: &mut Stack<String>) {
    let a = ds.pop().unwrap();
    let b = ds.pop().unwrap();
    ds.push(a);
    ds.push(b);
}

fn add(ds: &mut Stack<String>) -> Option<i64> {
    let a = ds.pop().unwrap().parse::<i64>().unwrap();
    let b = ds.pop().unwrap().parse::<i64>().unwrap();
    Some(a + b)
}

fn sub(ds: &mut Stack<String>) -> Option<i64> {
    let a = ds.pop().unwrap().parse::<i64>().unwrap();
    let b = ds.pop().unwrap().parse::<i64>().unwrap();
    Some(b - a)
}

fn mul(ds: &mut Stack<String>) -> Option<i64> {
    let a = ds.pop().unwrap().parse::<i64>().unwrap();
    let b = ds.pop().unwrap().parse::<i64>().unwrap();
    Some(a * b)
}

fn div(ds: &mut Stack<String>) -> Option<i64> {
    let a = ds.pop().unwrap().parse::<i64>().unwrap();
    let b = ds.pop().unwrap().parse::<i64>().unwrap();
    Some(b / a)
}

fn tokenize(input: String) -> Vec<String> {
    input.split(" ").map(|x| x.to_string()).collect()
}

fn execute_input<'t>(words: Vec<String>, cw: &HashMap<String, CW>, ds: &mut Stack<String>) -> Result<String> {
    let mut res = "NA".to_string();
    for w in words {
	if cw.contains_key(&w) {
	    res = match cw[&w] {
		CW::Dup => {dup(ds); "ok".to_string()},
		CW::Swap => {swap(ds); "ok".to_string()},
		CW::Add => {format!("{} ok", add(ds).unwrap())},
		CW::Sub => {format!("{} ok", sub(ds).unwrap())},
		CW::Mul => {format!("{} ok", mul(ds).unwrap())},
		CW::Div => {format!("{} ok", div(ds).unwrap())},
		CW::Dump => format!("{:?} ok", ds),
	    }
	} else {
	    ds.push(w.clone());
	    res = format!("{} ok", w);
	}
    }
    Ok(res)
}

fn main() -> Result<()> {
    let mut ds: Stack<String> = Stack::new();
    let cw = hashmap! {
	"swap".to_string() => CW::Swap,
	"dup".to_string() => CW::Dup,
	"dump".to_string() => CW::Dump,
	"+".to_string() => CW::Add,
	"-".to_string() => CW::Sub,
	"/".to_string() => CW::Div,
	"*".to_string() => CW::Mul,
    };
    print!("rforth> ");
    let _ = stdout().flush();
    for line in io::stdin().lock().lines() {
	let input = line.unwrap();
	let words = tokenize(input);
	let res = execute_input(words, &cw, &mut ds)?;
	println!("...{:#?}", res);
	print!("rforth> ");
	let _ = stdout().flush();
    }
    Ok(())
}

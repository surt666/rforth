use anyhow::Result;
use std::io::{self, stdout, Write, BufRead};
use std::{
    collections::{HashSet, HashMap},
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


fn dup(ds: &mut Stack<String>) {
    let a = ds.peek().unwrap();
    ds.push(a.to_string())
}

fn dot(ds: &mut Stack<String>) -> String {
    if ds.length() > 0 {
	let a: String = ds.pop().unwrap();
	a
    } else {
	"".to_string()
    }
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

fn save_new_word(words: Vec<String>, aw: &mut HashMap<String, Vec<String>>) -> Result<String> {
    let word_name = &words[1];
    let end_index = words.iter().position(|r| r.as_str() == ";").unwrap();
    let word_def = &words[2..end_index];
    aw.insert(word_name.clone(), word_def.to_vec());
    Ok(word_name.to_string())
}

fn substitue_words(words: Vec<String>, aw: &HashMap<String, Vec<String>>) -> Result<Vec<String>> {
    let mut swords = words.clone();
    let mut res: Vec<String> = vec![];
    for w in &swords {
	if aw.contains_key(w) {
	    let subst_index = swords.iter().position(|x| x == w).unwrap();
	    let s = &swords[0..subst_index];
	    let e = &swords[subst_index+1..];
	    let subst = &aw[w];
	    res = [s, &subst, e].concat();
	}
    }
    Ok(res)
}

fn execute_input(words: Vec<String>, cw: &HashSet<String>, aw: &mut HashMap<String, Vec<String>>, ds: &mut Stack<String>) -> Result<String> {
    let mut res = "NA".to_string();
    if words.first().unwrap() == ":" {
	return save_new_word(words, aw)
    }
    let subst_words: Vec<String> = substitue_words(words.clone(), aw)?;
    println!("SUBST {:?}", subst_words);
    for w in subst_words {
	if cw.contains(&w) {
	    res = match w.as_str() {
		"dup" => {dup(ds); "ok".to_string()},
		"swap" => {swap(ds); "ok".to_string()},
		"+" => format!("{} ok", add(ds).unwrap()),
		"-" => format!("{} ok", sub(ds).unwrap()),
		"*"  => format!("{} ok", mul(ds).unwrap()),
		"/" => format!("{} ok", div(ds).unwrap()),
		"dump" => format!("{:?} ok", ds),
		"." => format!("{} ok", dot(ds)),
		_ => "no match".to_string(),
	    }
	} else {
	    ds.push(w.clone());
	    res = format!("{:?} ok", w);
	}
    }
    Ok(res)
}

fn main() -> Result<()> {
    let mut ds: Stack<String> = Stack::new();
    let cw = hashset! {"swap".to_string(),"dup".to_string(),"dump".to_string(),"+".to_string(),"-".to_string(),"/".to_string(),"*".to_string(),".".to_string(),};
    let mut aw: HashMap<String, Vec<String>> = HashMap::new();
    print!("rforth> ");
    let _ = stdout().flush();
    for line in io::stdin().lock().lines() {
	let input = line.unwrap();
	let words = tokenize(input);
	let res = execute_input(words, &cw, &mut aw, &mut ds)?;
	println!("...{:#?}", res);
	print!("rforth> ");
	let _ = stdout().flush();
    }
    Ok(())
}

use anyhow::Result;
use std::io::{self, stdin, stdout, Write, BufRead};

fn main() {
    print!("rforth>");
    let _ = stdout().flush();
    for line in io::stdin().lock().lines() {
	println!("...{}", line.unwrap());
	print!("rforth>");
	let _ = stdout().flush();
    }
}

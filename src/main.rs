#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::path::Path;
use std::env;

fn print_flush(text: &str) -> io::Result<()>{
    try!(write!(io::stdout(), "{}", text));
    try!(io::stdout().flush());
    Ok(())
}


fn readline() -> io::Result<String> {
    let mut buffer = String::new();
    try!(io::stdin().read_line(&mut buffer));
    Ok(buffer)
}

fn shell() -> io::Result<()> {
    loop {
        try!(write_flush("eggs> "));
        let input = try!(readline());
        println!("{}", input.trim());
    }

    Ok(())
}

fn main() {
    let _ = shell();
}

use std::{fs, io::{self, IsTerminal, Read}, process};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    path: Option<String>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(args: Args) -> io::Result<()> {
    if args.path.is_none() && io::stdin().is_terminal() {
        return Err(io::Error::other("no input"));
    }

    let text = read_input(&args)?;
    let f_ease = coherity::flesch_reading_ease(&text);
    let fk_grade = coherity::flesch_kincaid_grade_level(&text);
    println!("Reading ease: {f_ease}\nFK Grade Level: {fk_grade}");
    Ok(())
}

fn read_input(args: &Args) -> io::Result<String> {
    match &args.path {
        Some(path) => fs::read_to_string(path),
        None => {
            let mut buf = String::new();
            io::stdin().lock().read_to_string(&mut buf)?;
            Ok(buf)
        }
    }
}
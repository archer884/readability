use std::{
    fs,
    io::{self, IsTerminal, Read},
    process,
};

use clap::Parser;
use coherity::Characterizer;

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
    let characterization = Characterizer::new().characterize(&text);

    let f_ease = characterization.reading_ease();
    let fk_grade = characterization.fk_grade_level();

    println!("Reading ease: {f_ease:.01}\nFK Grade Level: {fk_grade:.01}");
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

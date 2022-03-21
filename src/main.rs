extern crate clipboard_ext;
extern crate clap;

use std::fs;
use clipboard_ext::prelude::*;
use clipboard_ext::osc52::Osc52ClipboardContext;
use clap::Parser;
use std::io;
use std::io::Read;
use exitcode;


#[derive(Parser)]
#[clap(author, version, about = "Copy files' content to clipboard.", long_about = None)]
struct Cli {
    /// Optional files to operate on. '-' means `stdin` (Can only be used once).
    /// If no file is given, `stdin` is assumed.
    files: Vec<String>,
}


fn main() {
    let stdin = io::stdin();
    let cli = Cli::parse();
    let mut files: Vec<String> = cli.files;
    // println!("{:?}", files);

    let num_stdin_arg = files.iter().filter(|x| *x == "-").count();
    if num_stdin_arg > 1 {
        eprintln!("At most one stdin arg (-) be specified. ");
        std::process::exit(exitcode::USAGE);
    }

    if files.is_empty() {
        files.push("-".into());
    }

    let mut contents = String::new();
    for filename in &files {
        let mut tmp = String::new();
        if "-" == filename {
            stdin.lock().read_to_string(&mut tmp)
                .expect("Read from stdin failed");
        } else {
            tmp = fs::read_to_string(filename)
                .expect(&*format!("Something went wrong reading the file '{}'", filename));
        }

        if !contents.ends_with('\n') && !contents.is_empty() {
            contents.push('\n');
        }
        contents.push_str(&tmp);
    }

    let mut ctx = Osc52ClipboardContext::new().unwrap();
    ctx.set_contents(contents).unwrap();
}

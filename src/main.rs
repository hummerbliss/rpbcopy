extern crate clap;

use std::fs;
use clap::Parser;
use std::io;
use std::io::Read;
use std::io::Write;
use exitcode;


#[derive(Parser)]
#[clap(author, version, about = "Copy files' content to clipboard.", long_about = None)]
struct Cli {
    /// Optional files to operate on. '-' means `stdin` (Can only be used once).
    /// If no file is given, `stdin` is assumed.
    files: Vec<String>,
}

/// Send content to clipboard using OSC52 escape sequence
/// Works with iTerm2, Ghostty, and other terminals that support OSC52
fn copy_to_clipboard_osc52(content: &str) -> Result<(), Box<dyn std::error::Error>> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    
    // Encode the content in base64
    let encoded = STANDARD.encode(content.as_bytes());
    
    // Output OSC52 escape sequence: ESC ] 52 ; c ; <base64> BEL
    // \033 is ESC, \a is BEL (ASCII 7)
    print!("\x1b]52;c;{}\x07", encoded);
    io::stdout().flush()?;
    
    Ok(())
}


fn main() {
    let stdin = io::stdin();
    let cli = Cli::parse();
    let mut files: Vec<String> = cli.files;

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

    copy_to_clipboard_osc52(&contents)
        .expect("Failed to copy to clipboard");
}

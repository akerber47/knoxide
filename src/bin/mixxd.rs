extern crate knoxide;

// use knoxide::mix_fmt;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flag_r = false;
    let mut flag_h = false;
    let mut flag_d = false;
    let mut infile: Option<&String> = None;
    let mut outfile: Option<&String> = None;

    // Poor man's getopt
    for arg in args[1..].iter() {
        if arg == "-h" || arg == "--help" {
            flag_h = true;
        }
        else if arg == "-r" || arg == "--reverse" {
            flag_r = true;
        }
        else if arg == "-d" || arg == "--debug" {
            flag_d = true;
        }
        else if infile == None {
            infile = Some(arg);
        } else if outfile == None {
            outfile = Some(arg);
        } else {
            eprintln!("mixxd: Too many arguments!");
            std::process::exit(1);
        }
    }

    if flag_h {
        println!("Usage: mixxd [-r] [-v] [infile] [outfile]");
        println!("-r: reverse operation (convert dump to binary)");
        println!("-v: verbose operation");
        std::process::exit(2);
    }

    // If '-' is passed as infile or outfile, use stdin/stdout instead.
    if let Some(s) = infile {
        if s == "-" {
            infile = None;
        }
    }
    if let Some(s) = outfile {
        if s == "-" {
            outfile = None;
        }
    }

    if flag_r {
        panic!("Not yet implemented");
    } else {
        let mut in_bytes: Vec<u8> = vec![];
        match infile {
            Some(s) => {
                let mut f = std::fs::File::open(s).expect(
                    "mixxd: Failed to open file!");
                if let Err(e) = f.read_to_end(&mut in_bytes) {
                    eprintln!("mixxd: read failed! {}", e);
                }
            },
            None => {
                if let Err(e) = std::io::stdin().read_to_end(&mut in_bytes) {
                    eprintln!("mixxd: read failed! {}", e);
                }

            },
        };

        // Not yet implemented
        // let dump = mix_fmt::fmt_words(&in_bytes);
    }
    println!("{:?}", args);
}

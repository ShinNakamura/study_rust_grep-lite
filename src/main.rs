use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex, only_matched_text: bool) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(m) => {
                if only_matched_text {
                    println!("{}", m.as_str());
                } else {
                    println!("{}", line);
                }
            }
            None => (),
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for pattern")
        .arg(
            Arg::new("only_matched_text")
                .help("Print only matched text")
                .short('o')
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("File to search")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let only_matched_text = args.is_present("only_matched_text");

    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re, only_matched_text);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re, only_matched_text);
    }
}

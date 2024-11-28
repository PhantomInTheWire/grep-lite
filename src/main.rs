use clap::{Arg, Command};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for.")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("File to search.")
                .default_value("-"),
        )
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let input = args.get_one::<String>("input").unwrap();
    let re = Regex::new(pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();

        process_lines(reader, &re);
    } else {
        let file = File::open(input).expect("Failed to open file");
        let reader = BufReader::new(file);

        process_lines(reader, &re);
    }
}

fn process_lines<T: BufRead>(reader: T, re: &Regex) {
    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        if re.is_match(&line) {
            println!("{}", line);
        }
    }
}

use std::fs::File;
use std::io;
use std::io::BufReader ;
use std::io::prelude::* ;
use regex::{Regex};
use clap::{App, Arg};

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            None => (),
            Some(_) => println!("{}", line),
        }
    }
}

fn main() {
     let args = App::new("bankcsv")
         .version("0.1")
         .about("Searches for patterns")
         .arg(Arg::with_name("input")
             .help("file to search")
             .takes_value(true)
             .required(false))
         .get_matches();


    let input = args.value_of("input").unwrap_or("-");

    if input == "-" {
        let stdin = io::stdin() ;
        let reader = stdin.lock();
        process_lines(reader, re) ;
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader,re) ;
    }
}

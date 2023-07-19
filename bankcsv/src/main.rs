mod bank_statement;
use bank_statement::BankStatement;
use csv;
use std::error::Error;
use std::{fs, process};

// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path).expect("file missing");
    let mut reader = csv::Reader::from_reader(contents.as_bytes());

    for result in reader.deserialize() {
        let record: BankStatement = result?;
        println!("{:?}", record.amount);
    }

    Ok(())
}
fn main() {
    if let Err(e) = read_from_file("./data/checking_with_header.csv") {
        println!("Error reading file: {}", e);
        process::exit(1);
    }
}

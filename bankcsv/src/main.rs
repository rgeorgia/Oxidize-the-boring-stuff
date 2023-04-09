mod model;
mod dictionary;

extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
//#[macro_use]
extern crate serde_derive;
use csv::Error;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

use model::{BankStatement, AMOUNT, CHECK_NUMBER, DATE, RAW_PAYEE};

#[derive(Debug, StructOpt)]
#[structopt(name = "bankcsv", about = "Read and clean bank statement csv file")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str), long)]
    output: Option<PathBuf>,

    /// Activate verbose mode
    #[structopt(long)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    println!("{:?}", opt.output);
    println!("{:?}", opt.input);

    let csv_file = fs::read_to_string(opt.input)?;
    let mut bank_records = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_file.as_bytes());
    for record in reader.records() {
        let record = record?;
        let bank_row = BankStatement {
            date: String::from(&record[DATE]),
            amount: record[AMOUNT].parse::<f32>().unwrap(),
            check_number: String::from(&record[CHECK_NUMBER]),
            raw_payee: String::from(&record[RAW_PAYEE]),
            payee: String::from("payee"),
            category: String::from("category"),
            short_name: Some(String::from("short name"))
        };
        bank_records.push(bank_row)
    }
    println!("len of csv file: {:?}", csv_file.split('\n').count());
    println!("len of bank records is: {}", bank_records.len());
    println!(
        "len of bank records capacity is: {}",
        bank_records.capacity()
    );
    println!("{:?}", bank_records[12].category);
    println!("$ {}", bank_records[12].dollar_amount());

    Ok(())
}

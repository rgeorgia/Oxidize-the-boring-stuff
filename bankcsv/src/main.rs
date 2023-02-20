mod model;

extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
//#[macro_use]
extern crate serde_derive;
use csv::Error;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

use model::{BankStatement, AMOUNT, CHECKNUMBER, DATE, RAW_PAYEE};

#[derive(Debug, StructOpt)]
#[structopt(name = "bankcsv", about = "Read and clean bank statement csv file")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    //// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    /// Activate verbose mode
    #[structopt(long)]
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let csv_file = fs::read_to_string(opt.input)?;
    let mut bank_records: Vec<model::BankStatement> = Vec::with_capacity(csv_file.len());

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_file.as_bytes());

    for record in reader.records() {
        let record = record?;
        println!(
            "{} {} {} {}",
            &record[DATE], &record[AMOUNT], &record[CHECKNUMBER], &record[RAW_PAYEE]
        );
    }
    println!("len {}", csv_file.len());

    Ok(())
}

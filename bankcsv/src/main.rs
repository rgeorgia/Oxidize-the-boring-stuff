mod model;

extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TransactionEvent {
    date: String,
    amount: Option<f64>,
    star: String,
    check_number: String,
    payee: String,
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

/*
I had to add a header to the Checking2.csv and CreditCard4.csv, otherwise it did not work
Need to figure out how to do a ReaderBuilder from path then unwrap to use has_headers(false)
Here's what I did. I got the path to the file ( get_first_arg()? ), then read the contents into a String
then used ReaderBuilder::new() to control what is read.
Now I need to create a fn to read the contents are return the String
*/

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut file_data = File::open(file_path).unwrap();
    let mut contents = String::new();
    file_data.read_to_string(&mut contents).unwrap();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .double_quote(true)
        .flexible(true)
        .from_reader(contents.as_bytes());

    for result in rdr.deserialize() {
        let record: TransactionEvent = result?;
        println!("{}", record.date);
        println!("{}", record.payee);
        println!("{:?}", record.amount);
        println!("{}\n", record.check_number);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

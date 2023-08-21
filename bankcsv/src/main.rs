mod bank_statement;
use bank_statement::{BankStatement, RecordIndex};
use csv;
use std::error::Error;
use std::fs;

// https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html

fn bank_records(contents: String) -> Result<Vec<BankStatement>, Box<dyn Error>> {
    let mut records: Vec<BankStatement> = Vec::new();
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let mut index = 0;
    for record in reader.records() {
        let record = record?;
        index = index + 1;
        records.push(BankStatement {
            id: index,
            date: record[RecordIndex::Date as usize].to_string(),
            amount: record[RecordIndex::Amount as usize].to_string(),
            cleared: record[RecordIndex::Cleared as usize].to_string(),
            check_number: record[RecordIndex::CheckNumber as usize].to_string(),
            raw_payee: record[RecordIndex::RawPayee as usize].to_string(),
        });
    }

    Ok(records)
}

fn main() {
    let file_name: String = String::from("./data/Checking0723.csv");
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");
    let records = bank_records(contents);
    match records {
        Ok(r) => {
            for record in r.iter() {
                println!("{}: {}",record.id,record.amount) ;
            }
        },
        Err(e) => println!("Error parsing integer: {}", e),
    }
}

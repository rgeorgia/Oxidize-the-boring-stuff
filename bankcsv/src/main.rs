extern crate csv;
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
//#[macro_use]
extern crate serde_derive;
use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    println!("{:?}", opt.input);
    println!("{:?}", opt.output);
    println!("{}", opt.verbose);
}

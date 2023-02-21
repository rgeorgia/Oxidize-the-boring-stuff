///model.rs - module that models the data
extern crate serde;
// This lets us write `#[derive(Deserialize)]`.
#[allow(unused_imports)]
use serde_derive;

pub const DATE: usize = 0;
pub const AMOUNT: usize = 1;
pub const CHECKNUMBER: usize = 3;
pub const RAW_PAYEE: usize = 4;

#[allow(dead_code)]
pub enum Categories {
    Gas,
    Entertainment,
    Utility,
    Medical,
    Groceries,
    Spirits,
    Clothing,
    Other,
    Payment,
    Giving,
}

#[allow(dead_code)]
pub enum PaymentType {
    Payroll,
    Interest,
    Transfer,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BankStatement {
    pub date: String,
    pub amount: f32,
    pub check_number: String,
    pub raw_payee: String,
    pub payee: String,
    pub category: String,
}

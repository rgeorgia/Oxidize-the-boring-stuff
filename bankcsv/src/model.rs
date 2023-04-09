///model.rs - module that models the data

pub const DATE: usize = 0;
pub const AMOUNT: usize = 1;
pub const CHECK_NUMBER: usize = 3;
pub const RAW_PAYEE: usize = 4;
// pub const SHORT_NAME: usize = 5;
// pub const CATEGORY: usize = 6;

#[allow(dead_code)]
pub enum PaymentType {
    Payroll,
    Interest,
    Transfer,
}

#[derive(Debug)]
pub struct BankStatement {
    pub date: String,
    pub amount: f32,
    pub check_number: String,
    pub raw_payee: String,
    pub payee: String,
    pub category: String,
}

impl BankStatement {
    pub fn dollar_amount(&self) -> String {
        format!("{0:.2}", self.amount)
    }
    pub fn get_payee(raw_payee: String) -> String {
        "payee".to_string()
    }
}

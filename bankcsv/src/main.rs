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

fn main() {
    println!("Bank csv");
}

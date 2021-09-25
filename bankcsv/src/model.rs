///model.rs - module that models the data

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
}

pub enum PaymentType {
    Payroll,
    Interest,
    Transfer,
}

#[derive(Debug, Deserialize)]
pub struct CheckingRecord {
    date: String,
    amount: f64,
    check_number: String,
    payee: String,
}

#[derive(Debug, Deserialize)]
pub struct CreditCardRecord {
    date: String,
    amount: f64,
    payee: String,
}
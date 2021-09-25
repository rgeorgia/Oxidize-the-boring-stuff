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

#[derive(Debug)]
pub struct SpendingEvent {
    date: String,
    amount: f64,
    payee: String,
    category: Categories,
}


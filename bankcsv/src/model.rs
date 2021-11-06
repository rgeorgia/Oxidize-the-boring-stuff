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
#[serde(rename_all = "PascalCase")]
struct Checking {
    date: String,
    amount: Option<f64>,
    star: String,
    check_number: String,
    payee: String,
}

struct CreditCard {
    date: String,
    amount: Option<f64>,
    star: String,
    other: String,
    payee: String,
}
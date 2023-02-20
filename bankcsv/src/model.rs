///model.rs - module that models the data
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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BankStatement {
    date: String,
    amount: Option<f64>,
    star: String,
    check_number: String,
    raw_payee: String,
    payee: String,
    category: Categories,
}

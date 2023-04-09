/// dictionary.rs - contains a dictionary for short names and categories

#[allow(unused_imports)]
use std::collections::HashMap;

#[allow(dead_code)]
pub enum Categories {
    Groceries,
    Gas,
    Utilities,     // has sub categories
    Subscriptions, // has sub categories
    CreditCard,
    CarPayment,
    Prescriptions,
    Medical,
    Books,
    Entertainment,
    Insurance,
    Giving,
    Savings,
    Liquor,
    Other,
    Investments,
    Clothing,
    Interest,
    Pay,
    Deposit,
    HouseHold,
    Check,
    Mortgage,
}

impl Categories {
    pub fn get_by_name(&self, name: &str) -> Categories {
        match name {
            "FOOD LION" | "KROGER" | "KROGERS" => Categories::Groceries,
            "PENN NATIONAL" | "HOME SHIELD" => Categories::Insurance,
            "DUNKIN" | "DD" | "EMERALD COMMONS" | "FOOD COURT" | "CRACKER BARREL" => {
                Categories::Entertainment
            }
            "LIQUORS" | "ABC" => Categories::Liquor,
            "BRIDGECREST" => Categories::CarPayment,
            "CVS" => Categories::Prescriptions,
            "WAL-MART" | "DOLLAR GENERAL" => Categories::HouseHold,
            "PAYROLL" => Categories::Pay,
            "CHASE" | "VISA" => Categories::CreditCard,
            "AMERICAN FUNDS" => Categories::Investments,
            "ANCIENT FAITH" => Categories::Giving,
            "CHECK" => Categories::Check,
            "CONSUMER CELLULAR" => Categories::Subscriptions,
            "DUKE ENERGY" => Categories::Utilities,
            "MORTGAGE" => Categories::Mortgage,
            "ZIPS" => Categories::Gas,
            _ => Categories::Other,
        }
    }
}

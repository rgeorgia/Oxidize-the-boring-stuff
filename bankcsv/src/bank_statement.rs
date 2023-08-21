// use serde::Deserialize;
// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};

#[allow(dead_code)]
pub enum PaymentType {
    Payroll,
    Interest,
    Transfer,
}

// RecordIndex is an enum that maps to each field of the BankStatement struct
// I explicitly call out each number for readbility
pub enum RecordIndex {
    Date = 0,
    Amount = 1,
    Cleared = 2,
    CheckNumber = 3,
    RawPayee = 4,
    // Payee = 6,
    // Category = 7,
}

#[derive(Debug)]
pub struct BankStatement {
    // id will be a hash of date, amount and raw_payee
    // check_number is an Option because there a lot of times when check_number
    // will be an empty string
    pub id: i64,
    pub date: String,
    pub amount: String,
    pub cleared: String,
    pub check_number: String,
    pub raw_payee: String,
    // pub payee: String,
    // pub category: String,
}

// impl Hash for BankStatement {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         // need to convert amount from f64 to string because f64 does not have a hash trait
//         self.date.hash(state);
//         self.amount.to_string().hash(state);
//         self.raw_payee.hash(state);
//     }
// }

// impl BankStatement {
//     fn set_id<T: Hash>(t: &T) -> u64 {
//         let mut s = DefaultHasher::new();
//         t.hash(&mut s);
//         s.finish()
//     }

//     fn set_payee(&self) {
//         // search self.raw_payee for a regex match, if match set self.payee
//         // if no match then self.payee is "other"
//     }

//     fn set_category(&self) {
//         // search self.payee for a regex match, if match set self.category
//         // if no match then self.category is "other"
//     }
// }

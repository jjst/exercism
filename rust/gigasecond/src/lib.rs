extern crate chrono;
use chrono::*;

pub fn after(date: DateTime<UTC>) -> DateTime<UTC> {
    date.checked_add(Duration::seconds(1000000000))
        .expect("overflow in checked add")
}


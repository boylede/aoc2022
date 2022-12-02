pub mod days;

pub const LOOKUP_TABLE: &[(fn(&str)->(String,String), &str)] = &[(days::day1::run, days::day1::INPUT)];
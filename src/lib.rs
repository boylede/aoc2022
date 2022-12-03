pub mod days;
use days::*;

pub const LOOKUP_TABLE: &[(fn(&str) -> (String, String), &[&str])] = &[
    (day0, &[""]),
    (day1::run, day1::INPUTS),
    (day2::run, day2::INPUTS),
    (day3::run, day3::INPUTS),
];

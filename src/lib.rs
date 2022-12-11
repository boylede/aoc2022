pub mod days;
use days::*;

pub const LOOKUP_TABLE: &[(fn(&str) -> (String, String), &[&str])] = &[
    (day0, &[""]),
    (day1::run, day1::INPUTS),
    (day2::run, day2::INPUTS),
    (day3::run, day3::INPUTS),
    (day4::run, day4::INPUTS),
    (day5::run, day5::INPUTS),
    (day6::run, day6::INPUTS),
];

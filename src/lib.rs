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
    (day7::run, day7::INPUTS),
    (day8::run, day8::INPUTS),
    (day9::run, day9::INPUTS),
    (day10::run, day10::INPUTS),
    (day11::run, day11::INPUTS),
    (day12::run, day12::INPUTS),
];

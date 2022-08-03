#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

mod bank;
mod concurrency;
mod enum_sample;
mod error_handling;
mod hashmap;
mod ownership;
mod restaurant;
mod sample;
mod smart_pointer;
mod string_sample;
mod struct_example;
mod vector_sample;
use crate::restaurant::order_food;

fn main() {
    // sample::match_example();
    // string_sample::string_example();
    // vector_sample::example();
    // order_food();
    crate::bank::bank_example::example();
}

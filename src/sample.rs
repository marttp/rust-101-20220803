use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

pub fn part1() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello, {}! {}", name.trim_end(), greeting);
}

pub fn part2() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL)
}

// MAX function
pub fn part3() {
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
}

pub fn part4() {
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}

pub fn ternary_operator() {
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can Vote : {}", can_vote);
}

pub fn match_example() {
    let mut my_age = 47;
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You just gained the right to vote!"),
    };
}

pub fn arr_example() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
}

pub fn loop_example() {
    // ----- LOOP -----
    // Create an infinite loop that ends when break is called
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue; // Goes to beginning of loop
        }

        if arr_2[loop_idx] == 9 {
            break; // Breaks out of loop
        }

        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // ----- WHILE LOOP -----
    // Looping based on a condition
    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    // ----- FOR LOOP -----
    // For works better for cycling through collections
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}

pub fn tuple_example() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);

    let (v1, _v2, _v3) = my_tuple;
    println!("Age : {}", v1);
}

pub fn casting() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    // Cast using as
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    println!("Result: {}", int3_u32);
}
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // Ex:1 - ##########################################
    // println!("What is your name? ");
    // let mut name: String = String::new();
    // let greeting = "Nice to meet you";

    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't recieve Input");

    // println!("Hello {}! {}", name.trim_end(), greeting);

    // ##########################################

    // Ex:2 - ##########################################
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age = "47";
    // let mut age: u32 = age.trim().parse().expect("Age wasn't assign a number");
    // age = age + 1;
    // println!("I'm {} and i want ${}", age, ONE_MIL);

    // ##########################################

    // Ex:3 - ##########################################
    // Unsigned integer : u8, u16, u32, u64, u128, usize
    // Signed integer : i8, i16, i32, i64, i128, isize
    // println!("Max i32: {}", u32::MAX);
    // println!("Max i64: {}", u64::MAX);
    // println!("Max usize: {}", usize::MAX);
    // println!("Max u128: {}", u128::MAX);
    // println!("Max f32: {}", f32::MAX);
    // println!("Max f64: {}", f64::MAX);

    // ##########################################

    // Ex:4 - ##########################################
    // let is_true = true;
    // variables starting with _ are ignored by rust compiler when not used
    // let my_grade = 'A';

    // ##########################################

    // Ex:5 - ##########################################
    // let num_1: f32 = 1.111111111111111;
    // println!("f32 : {}", num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f64 : {}", num_2 + 0.111111111111111);
    // let mut num_3: u32 = 5;
    // let num_4: u32 = 4;

    // println!("5 + 4 = {}", num_3 + num_4);
    // println!("5 - 4 = {}", num_3 - num_4);
    // println!("5 * 4 = {}", num_3 * num_4);
    // println!("5 / 4 = {}", num_3 / num_4);
    // println!("5 % 4 = {}", num_3 % num_4);
    // num_3 += 1;

    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random Number: {}", random_num);

    // ##########################################

    // Ex:6 - ##########################################
    // let age = 8;
    // if (age >= 1) && (age <= 18) {
    //     println!("Important birthday");
    // } else if (age == 21) || (age == 50) {
    //     println!("Important birthday");
    // } else if age >= 65 {
    //     println!("Important birthday");
    // } else {
    //     println!("Not important birthday");
    // }
    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 { true } else { false };
    // println!("Can Vote: {}", can_vote);

    // ##########################################

    // Ex:7 - ###################################
    // let age2 = 8;
    // match age2 {
    //     1..=18 => println!("Important birthday"),
    //     21 | 50 => println!("Important birthday"),
    //     65..=i32::MAX => println!("Important birthday"),
    //     _ => println!("Not important birthday"),
    // };

    // let my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age) {
    //     Ordering::Less => println!("Too young to vote"),
    //     Ordering::Greater => println!("Old enough to vote"),
    //     Ordering::Equal => println!("Have fun voting"),
    // };

    // ##########################################

    // Ex:8 - ##########################################
    // let arr_1 = [1, 2, 3, 4];
    // println!("1st: {}", arr_1[0]);
    // println!("Length: {}", arr_1.len());

    // let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let mut loop_idx = 0;

    // loop {
    //     if arr_2[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }

    //     if arr_2[loop_idx] == 9 {
    //         break;
    //     }

    //     println!("Odd: {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }

    // while loop_idx < arr_2.len() {
    //     println!("Arr: {}", arr_2[loop_idx]);
    //     loop_idx += 1;
    // }

    // for val in arr_2.iter() {
    //     println!("Val: {}", val);
    // }

    // let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    // println!("Name : {}", my_tuple.1);
    // let (v1, v2, v3) = my_tuple;
    // println!("Age : {}", v1);

    // ##########################################

    // Ex:9 - ##########################################
    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str("Word");

    // for word in st1.split_whitespace() {
    //     println!("{}", word);
    // }

    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    // ##########################################

    // Ex:10 - ##########################################

    // let st3: String = String::from("x r t b h k k a m c");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }

    // let st4: &str = "Random string";
    // let mut st5: String = st4.to_string();
    // println!("{}", st5);

    // let byte_arr: &[u8] = st5.as_bytes();
    // let st6 = &st5[0..6];
    // println!("Length {}", st6.len());
    // st5.clear();
    // let st6: String = String::from("Just some");
    // let st7: String = String::from("Words");
    // let st8: String = st6 + &st7;

    // for char in st8.bytes() {
    //     println!("{}", char);
    // }

    // ##########################################

    // Ex:11 - ##########################################
    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // ##########################################

    // Ex:12 - ##########################################
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today a weekend? {}", today.is_weekend());

    // ##########################################
}

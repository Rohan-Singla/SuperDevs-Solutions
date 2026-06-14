use core::f32;
use std::{collections::HashMap, f64::consts::PI, fmt::{self, format}, ops::Add, sync::{Arc, Mutex}, task::{Context, Poll}, thread};

mod centralized_exchange;
mod week_1;
mod week_2;
mod solana_validator;

fn main() {

    // let result = sum_to(5);
    // let result = transform("2");

    // let result = multiply(2, 2);

    // let result: (i32,i32) = swap(2,4);

    // let result = data_types::first_last(&[4,5,6,7,7,8]);

    // let result = max_of_three(1, 2, 3);

    // let result = abs_value(-5);

    // let result = week_1::functions_controlflow::factorial(5);

    // println!("{}",result);

    // let result = functions_controlflow::fizbuzz(9);

    // let vector = vec![1,2,3,4];

    // let result = sum_vec(&vector);

    // println!("{:?}",result);

    // let mylist = List::Cons(1, Box::new(
    //     List::Cons(2, Box::new(
    //         List::Cons(3, Box::new(
    //             List::Cons(4, Box::new(
    //                 List::Cons(5, Box::new(List::Nil))
    //             ))
    //         ))
    //     ))
    // ));
    // let result = list_sum(&mylist);
    // println!("{}", result);

    // let result = count_owners(5);

    // println!("{}",result);

    // fn hash_chain(start_hex: &str, iterations: u32) -> String {
    //     let mut hash = hex_to_bytes(start_hex);
    //     for _ in 0..iterations {
    //         hash = sha256(&hash).to_vec();
    //     }
    //     hex(&hash)
    // }

    // fn verify_chain(start_hex: &str, claimed_end_hex: &str, iterations: u32) -> bool {
    //     let mut hash = hex_to_bytes(start_hex);
    //     for _ in 0..iterations {
    //         hash = sha256(&hash).to_vec();
    //     }
    //     hex(&hash) == claimed_end_hex
    // }
    
}

// tuple struct


/*
Vectors zijn aanpasbare arrays (size aanpassen).
*/

use std::mem;

pub fn run() {
    let mut nrs: Vec<i32> = vec![1, 2, 3, 4];
    // let nrs: [i32; 5] = [1, 2, 3]; Foutmelding aangezien deze arrray 4 lang is.

    // Change a value in the arr:
    nrs[2] = 20;

    // Add on to vector:
    nrs.push(5);
    nrs.push(6);

    // Pop off last value:
    nrs.pop();

    println!("{:?}", nrs);
    
    // get single value:
    println!("Single value: {}", nrs[0]);

    // get vector length:
    println!("Vector Length: {}", nrs.len());

    // vectors are stack allocated
    println!("Vector occupies: {} bytes", mem::size_of_val(&nrs));

    // get slice
    let slice: &[i32] = &nrs[1..3]; // Eerste x elementen
    println!("Slice: {:?}", slice);

    // Loop through vector values:
    for x in nrs.iter() {
        println!("Nr: {}", x);
    }

    println!("Before editing vector: {:?}", nrs);

    // Loop and mutate values;
    for x in nrs.iter_mut() {
        *x *=2;
    }

    println!("After editing vector: {:?}", nrs);
}
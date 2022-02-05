/*
Arrays zijn vaste lengtes waar alle elementen dezelfde type hebben.
*/

use std::mem;

pub fn run() {
    println!();
    println!("*********************");
    println!("***** arrays.rs *****");
    println!("*********************");

    let mut nrs: [i32; 5] = [1, 2, 3, 4, 5];
    // let nrs: [i32; 5] = [1, 2, 3, 4]; Foutmelding aangezien deze arrray 4 lang is.

    // Change a value in the arr:
    nrs[2] = 20;

    println!("{:?}", nrs);
    
    // get single val:
    println!("Single value: {}", nrs[0]);

    // get arr-len:
    println!("arr len: {}", nrs.len());

    // arrays are stack allocated
    println!("Array occupies: {} bytes", mem::size_of_val(&nrs));

    // get slice
    let slice: &[i32] = &nrs[0..3]; // Eerste x elementen
    println!("Slice: {:?}", slice);
}
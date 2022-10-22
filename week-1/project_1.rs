fn main() {
    let p = 250000;
    let r = 10;
    let n = 5;
    let a = p*(1+(r/100))*n;
    let ci = a-p;
    println!("total amount is {}",a);
    println!("compound interest is {}",ci);
 }
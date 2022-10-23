//project_2
//listing variables
fn main() {
    let p = 520000.00;
    let r = 10.00;
    let n = 5.00;
    let base = 1.00+r/100.00;
//f32 is used for floating point numbers
    let base = base as f32;
//powf is used for raising floating point numbers to a power    
    let a = p*(base.powf(n));
    let ci = a-p;
//printing amount and interest
    println!("total amount is {}naira",a);
    println!("compound interest is {}naira",ci);
 }
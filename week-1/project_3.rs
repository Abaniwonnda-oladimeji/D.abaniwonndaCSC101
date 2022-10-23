//project 3
fn main() {
    let p = 210000.00;
    let r = 5.00;
    let n = 3.00;
    let base :f32 = 1.00-r/100.00;
    let a = p*(base.powf(n));
    println!("value of the tv after 3 yrs is {}naira",a);
}    
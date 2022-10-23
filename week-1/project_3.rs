//project 3
// listing variables
fn main() {
    let p = 210000.00;
    let r = 5.00;
    let n = 3.00;
    let base :f32 = 1.00-r/100.00;
// calculating the value of the tv after 3 yrs 
    let a = p*(base.powf(n));
//printing the value of the tv after 3 yrs
    println!("value of the tv after 3 yrs is {}naira",a);
}    
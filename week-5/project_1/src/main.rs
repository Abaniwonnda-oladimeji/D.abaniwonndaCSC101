use std::io;
fn main() {
    println!("enter A");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter your A value: ");
    io::stdin().read_line(&mut a).expect("not a valid number");
    let a:f32 = a.trim().parse().expect("Not a valid number");

    println!("Enter your B value: ");
    io::stdin().read_line(&mut b).expect("not a valid string");
    let b:f32 = b.trim().parse().expect("not a valid number");

    println!("Enter your c value: ");
    io::stdin().read_line(&mut c).expect("not a valid string");
    let c:f32 = c.trim().parse().expect("not a valid number");

    let s:f32 = (b.powf(2.0))-(4.0*a*c);
    println!("b squared - 4ac is: {}",s);

    let mut first_root:f32 = (-b + (b.powf(2.0)-4.0*a*c).powf(0.5))/(2.0*a);
    println!("your first_root is: {}",first_root);

    let mut second_root:f32 = (-b - (b.powf(2.0)-4.0*a*c).powf(0.5))/(2.0*a);
    println!("your second_root is: {}",second_root);
    let second_root = second_root.abs();
}

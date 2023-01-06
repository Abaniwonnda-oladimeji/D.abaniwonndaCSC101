use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("how many years of experience do you have: ");
    io::stdin().read_line(&mut input1).expect("not a valid number");
    let input1:f32 = input1.trim().parse().expect("not a valid number");

    println!("what is your age: ");
    io::stdin().read_line(&mut input2).expect("not a valid number");
    let input2:f32 = input2.trim().parse().expect("not a valid number");

    if input1 >= 5.0 && input2 >= 40.0
    {
        println!("your annual incentive is 1,560,000naira ");
    }
    else if input1 > 5.0 && input2 >= 30.0 && input2 < 40.0
    {
        println!("your annual incentive is 1,480,000naira");
    }
    else if input1>5.0 && input2 < 28.0
    {
        println!("your annual incentive is 1,300,000naira");
    }
    else{
        println!("your annual incentive is 100000");
        println!("you are not experienced");
    }
}

fn main() {
    let n1 = "electrical".to_string();
    let n2 = " electronic".to_string();
    let n3 = " engineering".to_string();
    let n4 = n1 +  &n2 +  &n3; //n2 &n3 reference is passed

    //about electronic electrical engineering
    println!("\nThe {} is informed by the aspriation to train electrical/electronic
    engineering professionals in the areas of design, building and maintenance of 
    electrical control systems.",n4);

    let w1 = "computer".to_string();
    let w2 = " science".to_string();
    let w3 = w1 +&w2; //w2 reference is passed
    println!{};
    println!("{} is aimed at developing competent, creative,
    innovative, enterpreneural and ethically minded persons 
    capable of creating value in the diverse fields of 
    computer science. ",w3);



}

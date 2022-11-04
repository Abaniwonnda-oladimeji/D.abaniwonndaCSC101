fn main() {
    let fullname = "chibudum john umeh";
    let department = "computer science";
    let uni = "Pan-atlantic University";

    let mut school ="school of science".to_string();
    //push string
    school.push_str(" and technology");

    println!("My name is {}", fullname);
    //check length
    println!("the length of my fullname is: {}",fullname.len());
    println!("I am a student of {} Department",department);
    println!("{}",school);
    println!("{}",uni);
}

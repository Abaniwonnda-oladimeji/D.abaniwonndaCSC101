use std::io::Read;
use std::io;
fn main(){
    let mut input = String::new();
    println!("if you are an employee input 1");
    println!("if you are an administrator input 2");
    println!("if you are a project manager input 3");
    println!("if you are a vendor input 4");
    println!("if you are a customer input 5");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    

    if input.trim() == "1"{
        let mut file = std::fs::File::open("staff.SQL").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    
    }
    if input.trim() == "2"{
        let mut file = std::fs::File::open("globacomdb.SQL").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    if input.trim() == "3"{
        let mut file = std::fs::File::open("project.SQL").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    if input.trim() == "4"{
        let mut file = std::fs::File::open("Dataplan.SQL").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    if input.trim() == "5"{
        let mut file = std::fs::File::open("Customer.SQL").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}", contents);
    }
    
}


    
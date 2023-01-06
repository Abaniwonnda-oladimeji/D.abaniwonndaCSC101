use std::io;
fn main(){
	fn student_council_votex(){
		for i in 0..15{
			println!("what is your name {}",i);
			let mut name = String::new();
			io::stdin().read_line(&mut name).expect("invalid input");

			println!("what is your email");
			let mut email = String::new();
			io::stdin().read_line(&mut email).expect("invalid input");

			println!("what is your department");
			let mut department = String::new();
			io::stdin().read_line(&mut department).expect("invalid input");

			println!("what is your state of origin");
			let mut origin = String::new();
			io::stdin().read_line(&mut origin).expect("invalid input");

			println!("if your are a class rep input Y, if you are not a class rep input N");
			let mut rep = String::new();
			io::stdin().read_line(&mut rep).expect("invalid input");

			println!("if you are in 100 level input N, else input Y");
			let mut level = String::new();
			io::stdin().read_line(&mut level).expect("invalid input");

			
            println!("what is your cgpa");
			let mut cgpa = String::new();
			io::stdin().read_line(&mut cgpa).expect("invalid input");
			let cgpa:f32 = cgpa.trim().parse().expect("invalid input");

			if rep.trim().to_lowercase() == "y" && cgpa >= 4.0 && level.trim().to_lowercase() == "y"{
				println!(" {} {} {} {} you are eligible to vote",name ,email, department, origin);
 			}
			else{
				println!("sorry you are not eligible to vote");
			}



		}
		
	}
	student_council_votex()





}
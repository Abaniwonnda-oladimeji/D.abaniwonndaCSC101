use std::io;
fn main(){
	println!("which division are you in ");
	println!("if you aare in public service input 1");
	println!("if you are in geopolotical zoning input 2");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("invalid input");
	if input1 == "1"{
		fn GeoPo_merger(){
			let nameofcommissioner = ["Aigbogun Alamba Daudu","Murtala Afeez Bandu","Okorocha Calistus Ogbona","Adewale jimoh Akanbi","osazuwa faith Eseye"];
			let ministry = ["Internal affairs","justice","Defense","power and steel","petroleum"];
			let geopoloticalzone = ["Southwest","Northeast","SouthSouth","Southwest","Southwest"];

			let num:f32 = 0;
			for i in 0..4{
				num += 1;
				println("{} {} {}",nameofcommissioner[num],ministry[num],geopoloticalzone[num]);
			}
		}
		GeoPo_merger()

	}


}
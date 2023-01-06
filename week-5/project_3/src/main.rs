use std::io;
fn main() {
	println!("p = poundo yam/edinkainko soup          -N3,200");
	println!("F = fried rice / chicken                -N3,000");
    println!("a = Amala/Ewedu soup                    -N2,500");
    println!("e = Eba/Egusi soup                      -N2,000");
    println!("w = White rice / Stew                   -N2,500");
    println!("order above 10,000 get a 5 percent discount!!!!");

    println!("what is your order ");
    let mut order = String::new();
    io::stdin().read_line(&mut order).expect("invalid order");
    
    println!("what quantity do you want");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("invalid quantity");
    let quantity:i32 = quantity.trim().parse().expect("inavlid quantity");

    if order.trim().to_lowercase() == "p"{
    	let mut price = 3200 * quantity;
    	if price >= 10000{
    		println!("your discounted price is {}",price*95/100 );
    	}
    	else {
    		println!("the price of your food is {}",price );
    		
    	}
    }
    else if order.trim().to_lowercase() == "f"{
    	let mut price = 3000 * quantity;
    	if price >= 10000{
    		println!("your discounted price is {}",price*95/100 );
    	}
    	else {
    		println!("the price of yoru food is {}",price );
    	}
    	
    }
    else if order.trim().to_lowercase() == "a"{
    	let mut price = 2500 * quantity;
    	if price >= 10000{
    		println!("your discounted price is {}",price*95/100 );
    	}
    	else {
    		println!("the price of yoru food is {}",price );
    	}
    }

    else if order.trim().to_lowercase() == "e"{
    	let mut price = 2000 * quantity;
    	if price >= 10000{
    		println!("your discounted price is {}",price*95/100 );
    	}
    	else {
    		println!("the price of yoru food is {}",price );
    	}
    }
    else if order.trim().to_lowercase() == "w"{
    	let mut price = 2500 * quantity;
    	if price >= 10000{
    		println!("your discounted price is {}",price*95/100 );
    	}
    	else {
    		println!("the price of your food is {}",price );
    	}
    }

    else{
    	println!("/nsorry we dont have {} available right now",order);
    }


}

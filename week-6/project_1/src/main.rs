//code to calculate areas and volumes
use std::io;
use std::f32;
fn main(){
    println!("c = volume of cube  ");
    println!("t = area of trapezium calculation");
    println!("r = area of rhombus calculation");
    println!("p = area of parallelogram formula");
    println!("C (capital letter c) = volume of cylinder formula");

    println!("what volume do you want to calculate ");
    let mut formula = String::new();
    io::stdin().read_line(&mut formula).expect("invalid input");

    if formula.trim() == "t"{

        fn trapezium(){
            println!("what is your altitude in cm: ");
            let mut altitude = String::new();
            io::stdin().read_line(&mut altitude).expect("invalid input");
            let altitude:f32 = altitude.trim().parse().expect("invalid input");

            println!("what is your base in cm: ");
            let mut base = String::new();
            io::stdin().read_line(&mut base).expect("invalid input");
            let base:f32 = base.trim().parse().expect("invalid input");

            println!("what is your height in cm: ");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("invalid input");
            let height:f32 = height.trim().parse().expect("invalid input");

            let area = (altitude + base) * height/2.0;
            println!("the area of your trapezium is {:.3}cm squared",area);
        }
        trapezium()
    }

    else if formula.trim() == "c"{

        fn cube(){
            println!("what is your length in cm: ");
            let mut length = String::new();
            io::stdin().read_line(&mut length).expect("invalid input");
            let length:f32 = length.trim().parse().expect("invalid input");

            let volume = length.powf(3.0);
            println!("the volume of your cube is {:.3}cm cubed",volume);
            
        } 
        cube()
    }
    else if formula.trim() =="r"{
        fn rhombus(){
            println!("what is your first diagonal in cm: ");
            let mut diagonal_1 = String::new();
            io::stdin().read_line(&mut diagonal_1).expect("invalid input");
            let diagonal_1:f32 = diagonal_1.trim().parse().expect("invalid input");

            println!("what is your second diagonal in cm: ");
            let mut diagonal_2 = String::new();
            io::stdin().read_line(&mut diagonal_2).expect("invalid input");
            let diagonal_2:f32 = diagonal_2.trim().parse().expect("invalid input");

            let area = (diagonal_2 * diagonal_1)/2.0;
            println!("the area of your rhombus is {:.3}cm squared",area);

        }
        rhombus()
    }
    else if formula.trim() == "p" {
        fn parallelogram(){
            println!("what is your base: ");
            let mut base = String::new();
            io::stdin().read_line(&mut base).expect("not a valid input");
            let base:f32 = base.trim().parse().expect("not a valid input");

            println!("what is your height: ");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("not a valid input");
            let height:f32 = height.trim().parse().expect("not a valid input");

            let area = base * height;
            println!("the area of your parallelogram is {:.3}cm squared",area);

        }
        parallelogram()

    }
    else if formula.trim() == "C"{
        fn cylinder(){
            println!("what is your radius in cm: ");
            let mut radius = String::new();
            io::stdin().read_line(&mut radius).expect("not a valid input");
            let radius:f32 = radius.trim().parse().expect("not a valid input");

            println!("what is your height in cm: ");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("not a valid input");
            let height:f32 = height.trim().parse().expect("not a valid input");

            let r = f32::consts::PI;
            let volume = r * radius.powf(2.0) * height;
            println!("the volume of your cylinder is {:.3}cm cubed",volume);//TO ROUND TO 2DP


        }
        cylinder()
    }
    else{
        println!("sorry this program can not calculate that area");
    }

}



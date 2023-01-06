//method to print the get value
fn value(n:Option<&&str>)
{
    println!("Element of vector {:?}",n);

}
fn main() {
    let v = vec!["R","U","S","T","A","C","I","A","N"];

    let mut input1 = String::new();
    println!("\nEnter an index value between 0 to 8");
    std::io::stdin().read_line(&mut input1).expect("Faield to read input");

    //index is the non negative value which is smaller than the size of the vector
    let index:usize = input1.trim().parse().expect("invalid input");

    //getting value at given index value
    let ch: Option<&&str> = v.get(index);
    value(ch);
}

fn main() {
    //initalise a mutable tuple
    let mut mountain_heights = ("Everest",8848,"Fishtail",6990);

    println!("Original tuple = {:?}",mountain_heights);

    //change 3rd and 4th element of a mutable tuple
    mountain_heights.2 = "Lhotse";
    mountain_heights.3 = 8516;

    println!("changed tuple = {:?}",mountain_heights);

}

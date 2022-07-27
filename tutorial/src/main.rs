fn main() {
    let mut tup: (i32, bool, char) = (1, true, 'c');

    println!("tup is : {:?}", tup);

    tup.0 = 10;
    
    println!("tup at index 0 : {}", tup.0);

}

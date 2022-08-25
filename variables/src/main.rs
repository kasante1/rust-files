fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 9;
    println!("the value of x is: {}", x);

    let remainder = 43 % 5;
    println!("this the value of remainder: {}", remainder);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of x, y, z is: {}, {}, {}", x, y, z);

}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y:&'a str, ann: T) -> &'a str 
where T: Display{
    println!("announcement! {}", ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
fn main() {
    println!("Hello, world!");

    let string1 = "long string is long";
    let string2 = "xyz";

    let h  = longest_with_an_announcement(string1, string2, 7000);
    println!("{}",h);
}

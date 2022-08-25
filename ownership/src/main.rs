fn gives_ownership()->String{
    let some_string = String::from("ECG");
    some_string
}

fn takes_and_give_back(a_string:String)->String{
    a_string
}

fn calculate_length(s: String)->(String, usize){
    let length = s.len();

    (s, length)
}

fn calculate_length_reference(s: &String)-> usize{
    s.len()
}

fn main() {
     let mut s  = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("hello");

    let s2 = s1;

    println!("{}, world!++", s2);

    let s1  = gives_ownership();

    println!("{}",s1);

    let s2 = String::from("EPL");

    let s3 = takes_and_give_back(s2);
    println!("{}", s3);
    
    let s4 = String::from("get money!!!");

    let (s4, len) = calculate_length(s4);

    println!("the length of '{}' is {}.", s4, len);
    
    //referencing and borrowing

    let s5 = String::from("champions league ++");

    let len = calculate_length_reference(&s5);

    println!("the length of '{}' is {}", s5, len);
}

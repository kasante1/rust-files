use std::mem::*;

fn main() {
    println!("Hello, world!");

    // static strings

    let a: &str = "";
    let b: &str = "01234567";
    let c: &str = "abcde";

    println!("{} {} {}", size_of_val(a), size_of_val(b), size_of_val(c));

    // dynamic strings

    let mut a: String = "he".to_string();

    a.push('l');
    a.push('l');
    a.push('o');
    
    println!("{}", a);

    // create empty dynamic strings
    #[allow(unused_variables)]
    let s1 = String::new();
    let _s2 = String::from("");
    let _s3 = "".to_string();
    let _s4 = "".to_owned();
    let _s5 = format!("");

    let vs = ["hello", " ","world", "!"];
    let mut result = String::new();

    for s in vs{
        result += s;
    }
    println!("{}", result);

    println!("minimum element {}", min(&[23, 45,78,2,0,8,65,100,]));

    println!("minimum element {}", min(&[23, 45,78,]));

    // mutable arrays 

    let mut arr = [11,22,33,44];

    
    let sl_ref = &mut arr[1..3];
    println!("initial slice {:?}", sl_ref);
    sl_ref[1] = 0;
    println!("after the change of the second element {:?}", sl_ref);

    println!("last form of array {:?}", arr);

    print_codes("rising");

    print_nth_char("ghana", 3);

let mut v = vec![3, 4, 5];
let iterator: std::slice::IterMut<i32> = v.iter_mut();
for mut_item_ref in iterator {
    *mut_item_ref += 1;
}
println!("{:?}", v);

for item in vec![10, 20, 30].iter() {
    println!("{} ", *item + 1);
    }
    

    
}


fn min(arr: &[i32]) -> i32{
    let mut minimum = arr[0];
    for i in 1..arr.len(){
        if arr[i] < minimum{minimum = arr[i];}
    }
    return minimum
}

fn print_codes(s: &str){
    let mut iter = s.chars();

    loop{
        match iter.next(){
            Some(c) => {
                println!("{}", c);
            }
            None => {break;},
        }
    }
}

fn print_nth_char(s:&str, mut n:u32){
    let mut iter: std::str::Chars = s.chars();
    loop{
        let item:Option<char> = iter.next();

        match item{
            Some(c) => if n == 0 {println!("{}--{}", c, n); break;},
            None => {break;}
        }
        n -= 1;
    }
}
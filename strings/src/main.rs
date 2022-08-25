fn main() {
   let _s = String::new();

   let data =  "initial  contents";

   let s = data.to_string();

   println!("hello {}", s);
   
   //can word directly
   //let s = "initial contents".to_string();
   //or let s = String::from("initial contents");

   let s1 = String::from("hello, ");
   let s2 = String::from("world!");
   let s3 =  String::from(" Rust");

   let s = format!("{}-{}-{}", s1,s2,s3);

   let t = &s[3..6];

   println!("{}", t);

   for c in s.chars(){
       print!("{}",c);
   }

   use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{:?}", map);
}

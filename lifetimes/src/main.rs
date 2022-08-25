fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a>{
    part:&'a str,
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());

        println!("the longest string is {}", result);
    }
    
    let novel = String::from("call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().expect("could not find a '.'");

    let i=ImportantExcerpt{part: first_sentence};

    println!("{:?}", i);
}
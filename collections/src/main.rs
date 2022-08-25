
fn main(){
    
    let mut v = vec![1,2,3,4];

    v.push(5);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];

    println!("the third element is {}", third);
    println!("now v is {:?}", v);

    match v.get(10){
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element"),
    }
    for i in &v {
        print!("{},",i);
    }

    for i in &mut v{
        *i += 50;
        println!("{} **",*i);
    }

    println!("{:?}",v);

    // vector with different elements
    // in the form of an enum
    #[derive(Debug)]
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Int(4),
    ];

    for i in &row{
        println!("hello row {:?}", i);
    }
}
fn main() {
    println!("Hello, world!");

    f(&3.4);

    let contin = Continent::Asia;

    match contin{
        Continent::Europe => println!("E"),
        Continent::Asia => {let a = 7; println!("{}", a);},
        Continent::Africa => println!("AF"),
        Continent::America => println!("Am"),
        Continent::Oceania => println!("O"),
    }


    let outcome = Result::Failure(20, 'X');
    //let outcome = Result::Success(13);

    match outcome{
        Result::Success(0) => println!("Result: o"),
        Result::Success(1) => println!("Result: 1"),
        Result::Success(n) => println!("Result:{}", n),
        Result::Failure(10,'X' ) => println!("error: 10 X"),
        Result::Failure(10, m) => println!("Error 10 in module {}", m),
        Result::Failure(code,'X' ) => println!("error: n.{} X", code),
        Result::Failure(code, module) => println!("error n.{} in module {}", code, module),
        Result::Uncertainty => {},
    }


    for n in -2..5{
        println!("{} is {}.", n, match n{
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative",
            _ => "default catch all",
        });
    }
    
}

fn f(p:&f64){
    let a = Box::new(*p);
    {
        let b = Box::new([1,2,3]);
        println!("{}{:?}",*a,*b);
    }
    let c = Box::new(true);
    println!("{}{}", a, c);

    let factor = 2;

    #[allow(unused_variables)]
    let multiply = |a:i32| a*factor;

  

}

#[allow(dead_code)]

enum Continent{
    Europe,
    Asia,
    Africa,
    America,
    Oceania,
}

#[allow(dead_code)]
enum Result{
    Success(u8),
    Failure(u16, char),
    Uncertainty,
}
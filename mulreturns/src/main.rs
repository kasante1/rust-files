fn main() {
    println!("Hello, world!");
    // function return different or multiple types

    // print!("{} ", match f1() { E::E1 => 1, _ => -1 });
    // print!("{} ", f2().a);
    // print!("{} ", f3().0);
    // print!("{} ", f4()[0]);
    // print!("{} ", f5()[0]);

    print!("------------\n");

    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a;

    *p += 1;

    p = &mut b;
    *p += 1;
    
    println!("{}", *p);

    // let a:i16 = fgen::<i16>('a', 37, 41);

    // let b:f64 = fgen::<f64>('b', 37.2, 41.1);
    
    // println!("{}{}", a,b);

    // h('a', true);

    // let mut v = vec![11,22,33];

    // for _ in 0..5{
    //     let item:Option<i32> = v.pop();
    //     match item{
    //         Some(number) => println!("{}", number),
    //         None => println!("#, "),
    //     }
    // }

    show_divide(8., 2.);
    show_divide(8., 0.);

}

// function return multple types

// enum E {E1, E2}

// struct S {a:i32, b:bool}

// struct TS (f64, char);

// fn f1()->E{E::E2}
// fn f2()->S{S{a:49, b:true}}
// fn f3()->TS{TS(4.7,'w')}
// fn f4()->[i16;4]{[7,-2,0,19]}
// fn f5()->Vec<i64> {vec![12000]}

// generics


// fn fgen<T>(ch: char, num1:T, num2:T)-> T{
//     if ch == 'a'{num1}
//     else{num2}
// }

// fn h<Param1, Param2>(a: Param1, b: Param2){}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.{
        Err(format!("Divide by zero error"))
    }else{
        Ok(numerator/denominator)
    }
}

fn show_divide(num: f64, den:f64){
    match divide(num, den) {
        Ok(val)=> println!("{}/{} = {}", num, den, val),
        Err(msg) => println!("cannot divide {} by {}: {}", num, den, msg),
        
    }
}
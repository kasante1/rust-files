fn another_function(){
    println!("another function");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}

fn main() {
    println!("Hello, world!");
    another_function();
    let _x = 3;

    let y = {
        let x = 4;
        x + 1
    };

    println!("the value of y is: {}", y);

    let x = five();

    println!("the value of x is: {}", x);


    let g = plus_one(99);

    println!("the value of x is: {}", g);

    let mut counter = 0;
    
    let result = loop{
        counter += 1;

        if counter == 50{
            break counter * 2;
        }
    };
    println!("the result is {}", result);
    
}
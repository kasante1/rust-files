#[derive(Debug)]
#[allow(dead_code)]
enum UsState{
    Alabama,
    Alaska,
    Maryland,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_action(coin:Coin) -> u8{
    match coin{
        Coin::Penny => {
            println!("lucky penny");1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}!", state);
        25
    }
    }
}

fn main(){
    let you = value_in_action(Coin::Penny);
    
    let girl = value_in_action(Coin::Dime);

    let fight = value_in_action(Coin::Nickel);

    let volvo = value_in_action(Coin::Quarter(UsState::Maryland));

    println!("this is the value of {} and the other value is {}, {}, {}", you, girl, fight, volvo);
}
mod example_one;
mod learn_two;

use example_one::f;
use learn_two::hello::say_hi;

fn main() {


    println!("{:#?} {:#?} {:#?} {:#?}",f().0, f().1, f().2, f().3);

    say_hi();

    let n1 = 11;
    let result;
    {
        let n2 = 12;

        // fn func(_m1: &i32, _m2: &i32) -> &i32{
        //     _m1
        // }
        // result = func(&n1, &n2);
        result = 
        {
            let _m1 = &n1; // 11
            let _m2 = &n2; //12
            _m1 // 11
        }
    }

    print!("{}", *result);
}

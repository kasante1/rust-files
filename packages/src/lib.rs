mod front_of_house;

fn serve_order(){}

mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }

    fn cook_order(){}
}

//absolute path import
use crate::front_of_house::hosting;

//relative path import

// use self::front_of_house::hosting;


pub fn eat_at_restaurant(){

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("i would like to {} toast please", meal.toast);

    //println!("hello {}", meal.seasonal_fruit);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


mod back_of_house_resturant{
    #[derive(Debug)]
    pub enum Appetizer{
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_kozo(){
    let order1 = back_of_house_resturant::Appetizer::Soup;
    let order2 = back_of_house_resturant::Appetizer::Salad;

    println!("{:#?}{:#?}", order1, order2);
}
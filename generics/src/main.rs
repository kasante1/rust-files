fn largest (list: &[i32])-> i32{
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}


fn largest_char(list: &[char])->char{
    let mut largest = list[0];

    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}


//fn largest_gen<T>(list: &[T]) -> T{}


// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//     if item > largest {
//     largest = item;
//     }
//     }
//     largest
// }

// struct Point<T>{
//     x: T,
//     y:T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }

// }

// struct Joint<T, U>{
//     x:T,
//     y:U,
// }

// impl<T, U> Joint<T, U> {
//     fn mixup<V, W> (self, other: Joint<V, W>) -> Joint<T, U> {
//         Joint{
//             x: self.x,
//             y: self.y,
//         }

//     }
// }

pub trait Summary{
    fn summarize(&self) -> String;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self)-> String{
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let number_list = vec![34,50,25,100,65];
    
   let result = largest(&number_list);
   println!("the largest number is {}", result);
   
   let char_list = vec!['y','m','a','q'];

   let result  = largest_char(&char_list);

   println!("the largest char is {}", result);

//    let integer = Point{x:5, y:10};
//    let float = Point{x:1.0, y:4.0};
//    let char = Point{x:'g', y:'j'};

//    let both_integer_float = Joint{x:5, y:10};
//    let both_char_float = Joint{x:'c', y:9.0};

   let tweet = Tweet{
       username: String::from("horse_ebook"),
       content: String::from("of course as you probably already know people"),
       reply: false,
       retweet: false,
   };
   println!("1 new tweet: {}", tweet.summarize());
}

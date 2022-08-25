fn main() {
    println!("Hello, world!");

    // let person = Person{
    //     personal_names: "John".to_string(),
    //     family_names: "Doe".to_string(),
    // };

    // println!("{}", person.naming());

    println!("{}", Person("John".to_string(), 30).age());

    println!("{}", Visibility::Visible.is_not_visible());

    f();
}

// self is different from Self

// struct Person{
//     personal_names: String,
//     family_names: String,
// }

// impl Person{
//     fn naming(self)-> String{
//         format!("{}{}", self.personal_names, self.family_names)
//     }
// }


struct Person(String, u32);

#[allow(dead_code)]

enum Visibility{ Visible, Hidden, Collapsed}

impl Person{
    fn age(self)-> u32{
        self.1
    }
}

impl Visibility {
    fn is_not_visible(&self) -> bool{
        match self{
            Visibility::Visible => false,
            _ => true,
        }
    }
}

fn f(){
    println!("f1");
    g();
    m::f();
    m::m::f();
}

fn g(){println!("g1");}

mod m{
    pub fn f(){
        println!("f2");
        g();
        m::f();
        super::g();
    }
    fn g(){println!("g2");}
    pub mod m {
        pub fn f(){
            println!("f3");
            g();
            super::g();
            super::super::g();
            crate::g();
        }
        fn g(){println!("g3");}
    }
}
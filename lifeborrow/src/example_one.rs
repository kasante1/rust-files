pub static FOUR: u8 = 4;
pub fn f()-> (bool, &'static u8, &'static str, &'static f64){
    (true, &FOUR, "Hello", &3.14)
}

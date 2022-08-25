fn area (width: u32, height: u32) -> u32{
    width * height
}

fn areaa (dimensions:(u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn area3 (rectangle:&Rectangle) -> u32{rectangle.width * rectangle.height}


fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels", area(width1, height1));

    let rect1 = (30, 50);

    println!("the area of the rectangle is {} square pixels", areaa(rect1));

    let rect2 = Rectangle{width: 300, height: 500};

    println!("the area of the rectangle is {} square pixels", area3(&rect2));

    println!("rect2 is a {:#?}", rect2);
}

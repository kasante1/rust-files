fn main() {
    println!("{} {}", quartic_root(100f64),   quartic_root(100f32));

    println!("{} {}", abs_quartic_root(-100f64),
    abs_quartic_root(-100f32));

    println!("abs quartic roots {}", abs_quartic_roots(-100f64));
}

/*
trait is essentially a set of method signatures
a type is said to satisfy or to implement a trait
if it declares the body of all the methods specified by that trait 

trait is like interfaces in golang.
*/


trait HasSquareRoot{
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32{
    fn sq_root(self) -> Self {
        self.sqrt()
    }
}

impl HasSquareRoot for f64{
    fn sq_root(self) -> Self {
        self.sqrt()
    }
}


fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot{
        x.sq_root().sq_root()
    }

// traits with multiple methods

trait HasSqrtAndAbs{
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}

impl HasSqrtAndAbs for f32{
    fn sqrt(self) -> Self {self.sqrt()}
    fn abs(self) -> Self {self.abs()}
}

impl HasSqrtAndAbs for f64{
    fn sqrt(self) -> Self{self.sqrt()}
    fn abs(self) -> Self{self.abs()}
}

fn abs_quartic_root<Number>(x:Number) -> Number
where Number: HasSqrtAndAbs{
    x.abs().sqrt().sqrt()
}

/*every impl block used to implement a trait
must have the same signatures of the trait
it is implementing: not one more, not one less,
not one different  */ 

/*to promote loose coupling use the method below*/

trait HasSquareRoots{
    fn sqrt(self) -> Self;
}

impl HasSquareRoots for f32{
    fn sqrt(self) -> Self{self.sqrt()}
}

impl HasSquareRoots for f64{
    fn sqrt(self) -> Self {self.sqrt()}
}

trait HasAbsoluteValues{
    fn abs(self) -> Self;
}

impl HasAbsoluteValues for f32{
    fn abs(self) -> Self{ self.abs()}
}

impl HasAbsoluteValues for f64{
    fn abs(self) -> Self{ self.abs()}
}

fn abs_quartic_roots<Number>(x:Number) -> Number
where Number: HasSquareRoots + HasAbsoluteValues{
    x.abs().sqrt().sqrt()
}
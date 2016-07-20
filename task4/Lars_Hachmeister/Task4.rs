use std::ops::*;
use std::fmt::*;
use std::marker::Copy;

fn main() {
    println!("{:?}", summe(7, 5));
    let b = swagger { word: "test" };
    println!("{}", b);
}


fn summe<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    ((a + b), (a * b))
}

struct swagger<T> {
    word: T,
}

impl<T: Display> Display for swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.word)
    }
}

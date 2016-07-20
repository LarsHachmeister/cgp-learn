use std::ops::*;
use std::fmt::*;
use std::marker::Copy;

fn main() {
    println!("{:?}", summe(7, 5));
    // let b = swagger { word: "test" };
    // print!("{}", b);
}


fn summe<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    ((a + b), (a * b))
}

// struct swagger {
//     word: T,
// }

// impl Display for swagger {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "#swag {} #yolo)", self.word)
//     }
// }

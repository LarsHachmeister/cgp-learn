use std::ops::*;
use std::fmt::*;
use std::iter::Iterator;

fn main() {
    println!("{:?}", summe(7, 5));
    let b = Swagger { word: "test" };
    println!("{}", b);
    for i in Counter::new().take(20) {
        println!("{}", i);
    }

}


fn summe<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    ((a + b), (a * b))
}

struct Swagger<T> {
    word: T,
}



impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.word)
    }
}

struct Counter {
    current: u32,
    previous: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            current: 1,
            previous: 0,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.previous;
        self.previous = self.current;
        self.current = self.current + self.previous;
        Some(res)
    }
}

use std::fmt::{Display, Formatter, Result};

struct Swagger<T: Display> {
    obj: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.obj)
    }
}
fn main() {
    let my_string = Swagger { obj: "hallo" };
    println!("{}", my_string);
}

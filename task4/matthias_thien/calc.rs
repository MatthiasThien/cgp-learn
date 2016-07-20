use std::ops::{Mul, Add};

fn calc<T: Mul + Add + Copy>(eins: T, zwei: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    ((eins + zwei), (eins * zwei))
}

fn main() {
    println!("{:?}", calc(2, 3));
}

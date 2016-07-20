fn main() {
    for i in Fibonacci::new().take(20) {
        println!("{}", i);
    }
}
struct Fibonacci {
    o: u32,
    c: u32,
}
impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { o: 0, c: 1 }
    }
}
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.o + self.c;
        self.o = self.c;
        self.c = new;
        Some(self.o)
    }
}

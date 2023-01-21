use display_macro::display;

mod b {
    use super::*;
    #[display("Person {} is called {}", 0, 1)]
    struct Person(u32, &'static str);
}

pub fn main() {}

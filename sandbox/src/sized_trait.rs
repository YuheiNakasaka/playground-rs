fn say_xxx(message: &str) -> Box<&str> {
    Box::new(message)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_closure2() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let message = "Hello, world!";
    let resp = say_xxx(message);
    println!("{}", resp);
}

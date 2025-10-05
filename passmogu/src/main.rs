fn main() {
    println!("Hello, world!");
    let args: Vec<String> = std::env::args().collect();
    for arg in args {
        println!("{arg}");
    }
}

fn main() {
    let name = std::env::args()
        .skip(1)
        .next()
        .unwrap_or("world".to_string());
    println!("hello {}!", name);
}

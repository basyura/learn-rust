fn main() {
    println!("{}", f1("rust"));
}

fn f1(name: &str) -> String {
    let s = format!("Hello, {}!", name);
    return s;
}

fn main() {
    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");

    assert_eq!(format!("{:.2}", f), "4.40");

    let t = (1, "ABC");
    assert_eq!(format!("{:?}", t), r#"(1, "ABC")"#);

    let s1 = "42";
    assert_eq!(s1.parse::<i32>(), Ok(42));

    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    let s3 = "amazon";
    s3.to_uppercase();

    s3.to_string();
    println!("{}", s3.len());
}

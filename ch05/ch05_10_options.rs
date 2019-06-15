fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));

    assert_eq!(a1.get(4), None);

    let mut o1 = Some(10);
    match o1 {
        Some(s) => {
            assert_eq!(s, 10);
            println!("{}", "s == 10");
        }
        None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
        assert_eq!(s, 20);
        println!("{}", "s == 20");
    }

    let mut o2 = Some(String::from("Hello"));
    assert_eq!(o2.unwrap(), "Hello");

    //o2.unwrap(); -> Error
    let v = None.unwrap_or_else(|| {
        println!("{}", "unwrap or else");
        return String::from("o2 is none");
    });
    assert_eq!(v, "o2 is none");

    o2 = None;
    assert_eq!(
        o2.unwrap_or_else(|| {
            println!("{}", "unwrap or else");
            return String::from("o2 is none");
        }),
        "o2 is none"
    );

    let mut o3 = Some(25);
    assert_eq!(o3.map(|n| n * 10), Some(250));
    o3 = Some(10);
    o3.map(|n| n * 10)
        .and_then(|n| if n >= 200 { Some(n) } else { None });

    // 複数行で if を書く場合
    let o3_result = o3.map(|n| n * 10).and_then(|n| {
        if n >= 200 {
            return Some(n);
        } else {
            return None;
        }
    });
    // None が返るので -999999 が出力される
    println!("{}", o3_result.unwrap_or_else(|| -999999));
}

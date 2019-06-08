fn main() {
    // インデックスにあるので Some が返る
    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    // インデックスに無いので None が返る
    assert_eq!(add_elems(&[7, 11]), None);
}

fn add_elems(s: &[i32]) -> Option<i32> {
    // ? を使うと値がある場合は取り出し、無い場合は関数から出る (None を返す)

    let s0 = s.get(0)?;
    println!("{}", "s0 get?");
    let s3 = s.get(3)?;
    println!("{}", "s3 get?");
    return Some(s0 + s3);
}

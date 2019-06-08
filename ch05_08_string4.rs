fn main() {
    let utf16: Vec<u16> = vec![0x61, 0x62, 0x6f22, 0x5b57];
    if let Ok(s) = String::from_utf16(&utf16) {
        assert_eq!(s, "ab漢字");
        println!("{}", "OK");
    } else {
        unreachable!();
    }
}

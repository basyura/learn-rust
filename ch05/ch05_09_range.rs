fn main() {
    let a = ['a', 'b', 'c', 'd', 'e'];

    assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[..3], ['a', 'b', 'c']);
    assert_eq!(a[..=3], ['a', 'b', 'c', 'd']);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c']);
    assert_eq!(a[1..=3], ['b', 'c', 'd']);

    assert_eq!(a[2..4], ['c', 'd']);

    let b = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    assert_eq!(b[0..1], ['0']);
    assert_eq!(b[0..2], ['0', '1']);
    assert_eq!(b[0..=2], ['0', '1', '2']);
    assert_eq!(b[0..3], ['0', '1', '2']);

    // = 使って添え字の位置までを含むと考えるほうが分かりやすい
    assert_eq!(b[4..=5], ['4', '5']);

    assert_eq!(.., std::ops::RangeFull);
    assert_eq!(..3, std::ops::RangeTo { end: 3 });
    assert_eq!(..=3, std::ops::RangeToInclusive { end: 3 });
}

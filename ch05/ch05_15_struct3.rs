struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

fn main() {
    let vx0 = Vertex(0, 1);
    let vx1 = Vertex(2, 3);

    let triangle = Triangle(vx0, vx1, Vertex(4, 5));
    assert_eq!((triangle.0).0, 0);
    assert_eq!((triangle.0).1, 1);
    assert_eq!((triangle.1).0, 2);
    assert_eq!((triangle.1).1, 3);
    assert_eq!((triangle.2).0, 4);
    assert_eq!((triangle.2).1, 5);
}

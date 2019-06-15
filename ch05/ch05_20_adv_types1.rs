#[derive(Default)]
pub struct Polygon<T> {
    pub vertexes: Vec<T>,
}

trait Coordinates {}

#[derive(Default)]
struct CratesianCoord {
    x: f64,
    y: f64,
}
impl Coordinates for CratesianCoord {}

#[derive(Default)]
struct PolarCoord {
    r: f64,
    theta: f64,
}

impl Coordinates for PolarCoord {}

fn main() {
    let vertexes = vec![
        CratesianCoord { x: 0.0, y: 0.0 },
        CratesianCoord { x: 50.0, y: 0.0 },
        CratesianCoord { x: 30.0, y: 20.0 },
    ];

    let poly = Polygon {
        vertexes,
        ..Default::default()
    };

    println!("{}", "hi");
}

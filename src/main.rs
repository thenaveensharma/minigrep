struct Coordinates<T> {
    x: T,
    _y: T,
}

impl Coordinates<i32> {
    fn distance_from_origin(&self) -> i32 {
        self.x.pow(2) + self._y.pow(2)
    }
}
fn main() {
    let point1 = Coordinates { x: 1, _y: 2 };
    println!(
        "Point1 x is {} and distance from origin is {}",
        point1.x,
        point1.distance_from_origin()
    );
    let integer = Some(1);
    let integer = Some(1.0);
}

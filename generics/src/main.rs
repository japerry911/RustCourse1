fn main() {
    generics();
}

struct Point<T, V> {
    x: T,
    y: V
}

struct Point2<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point2<T>,
    end: Point2<T>
}

fn generics() {
    let a:Point2<u16> = Point2 { x: 0, y: 4 };
    let b:Point<f64, f64> = Point { x: 1.2, y: 3.4 };
    let c:Point<u16, f64> = Point { x: 1, y: 3.2 };
    let d:Point2<u16> = Point2 { x: 12, y: 1224 };

    let my_line:Line<u16> = Line { start: a, end: d };
}

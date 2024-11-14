#[derive(Debug, Clone, Copy)]
struct Point {
    x: Option<i64>,
    y: Option<i64>,
}

impl Point {
    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    fn infinity() -> Point {
        Point { x: None, y: None }
    }
}

struct EllipticCurve {
    a: i64,
    b: i64,
    p: i64, // The prime field over which the curve is defined
}

impl EllipticCurve {
    fn add_points(&self, p1: Point, p2: Point) -> Point {
        if p1.is_infinity() {
            return p2;
        }
        if p2.is_infinity() {
            return p1;
        }

        let (x1, y1) = (p1.x.unwrap(), p1.y.unwrap());
        let (x2, y2) = (p2.x.unwrap(), p2.y.unwrap());

        if x1 == x2 && y1 != y2 {
            return Point::infinity();
        }

        let lambda = if x1 == x2 && y1 == y2 {
            let numerator = (3 * x1 * x1 + self.a) % self.p;
            let denominator = mod_inverse(2 * y1, self.p);
            (numerator * denominator).rem_euclid(self.p)
        } else {
            let numerator = (y2 - y1).rem_euclid(self.p);
            let denominator = mod_inverse((x2 - x1).rem_euclid(self.p), self.p);
            (numerator * denominator).rem_euclid(self.p)
        };

        let x3 = (lambda * lambda - x1 - x2).rem_euclid(self.p);
        let y3 = (lambda * (x1 - x3) - y1).rem_euclid(self.p);

        Point {
            x: Some(x3),
            y: Some(y3),
        }
    }

    fn double_point(&self, p: Point) -> Point {
        if p.is_infinity() {
            return p;
        }

        let (x1, y1) = (p.x.unwrap(), p.y.unwrap());

        let lambda = {
            let numerator = (3 * x1 * x1 + self.a).rem_euclid(self.p);
            let denominator = mod_inverse((2 * y1).rem_euclid(self.p), self.p);
            (numerator * denominator).rem_euclid(self.p)
        };

        let x3 = (lambda * lambda - 2 * x1).rem_euclid(self.p);
        let y3 = (lambda * (x1 - x3) - y1).rem_euclid(self.p);

        Point {
            x: Some(x3),
            y: Some(y3),
        }
    }

    // A method to verify if a point is on the curve.
    fn is_point_on_curve(&self, p: Point) -> bool {
        if p.is_infinity() {
            return true;
        }

        let x = p.x.unwrap();
        let y = p.y.unwrap();
        (y * y).rem_euclid(self.p) == (x * x * x + self.a * x + self.b).rem_euclid(self.p)
    }
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (mut a, mut m) = (a % m, m);
    let (mut x0, mut x1) = (0, 1);

    while a > 1 {
        let q = a / m;
        let t = m;
        m = a % m;
        a = t;
        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }

    if x1 < 0 {
        x1 += m;
    }

    x1
}

fn main() {
    let curve = EllipticCurve { a: 4, b: 4, p: 313 };

    let p1 = Point { x: Some(274), y: Some(288) };
    let p2 = Point { x: Some(159), y: Some(45) };

    // Check if points are on the curve
    println!("Is p1 on the curve? {}", curve.is_point_on_curve(p1));
    println!("Is p2 on the curve? {}", curve.is_point_on_curve(p2));

    let result = curve.add_points(p1, p2);
    println!("p1 + p2 = {:?}", result);

    let result_double = curve.double_point(p1);
    println!("2 * p1 = {:?}", result_double);
}

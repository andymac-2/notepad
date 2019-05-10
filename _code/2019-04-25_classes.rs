#[derive(Debug)]
struct Circle {
    radius: f64
}

impl Circle {
    fn new_option (radius: f64) -> Option<Circle> {
        if radius > 0.0 {
            Some(Circle {radius: radius})
        }
        else {
            None
        }
    }

    fn new_assert (radius: f64) -> Circle {
        assert!(radius > 0.0);
        Circle {radius: radius}
    }

    fn grow (&mut self, length: f64) {
        self.radius += length;
    }

    fn shrink (mut self, length: f64) -> Option<Circle> {
        if self.radius - length > 0.0 {
            self.radius -= length;
            Some(self)
        }
        else {
            None
        }
    }
}

fn main () {
    let mut c = Circle::new_assert(10.0);
    // Circle { radius: 10.0 }
    println!("{:?}", c);

    c.grow(5.0);
    // Circle { radius: 15.0 }
    println!("{:?}", c);

    // None
    println!("{:?}", c.shrink(20.0));
}
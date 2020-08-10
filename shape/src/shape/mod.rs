#[derive(Debug)]
pub struct Circle {
    pub radius: f32
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub trait Shape {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
    fn area_per_perimeter(&self) -> f64 {
        dbg!(std::any::type_name::<Self>());
        dbg!(std::mem::size_of::<&Self>());
        dbg!(std::mem::size_of_val(self));
        self.area() / self.perimeter()
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2) as f64
    }

    fn perimeter(&self) -> f64 {
       std::f64::consts::PI * self.radius as f64 * 2.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

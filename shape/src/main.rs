mod shape;
use shape::{Circle, Rectangle, Shape};


fn area_per_perimeter(shape: &dyn Shape) -> f64 {
    dbg!(std::any::type_name::<dyn Shape>());
    dbg!(std::mem::size_of::<&dyn Shape>());
    dbg!(std::mem::size_of_val(shape));
    shape.area() / shape.perimeter()
}

fn main() {

    let c1 = Circle { radius : 1.0 };
    let c2 = Circle { radius : 2.5 };

    let r1 = Rectangle { width: 3.0, height: 3.0};
    let r2 = Rectangle { width: 4.0, height: 6.0};

    println!("{:?}: {}, {}, {}", c1, c1.area(), c1.perimeter(), c1.area_per_perimeter());
    // println!("{:?}: {}, {}, {}", c2, c2.area(), c2.perimeter(), c2.area_per_perimeter());
    // println!("{:?}: {}, {}, {}", r1, r1.area(), r1.perimeter(), r1.area_per_perimeter());
    // println!("{:?}: {}, {}, {}", r2, r2.area(), r2.perimeter(), r2.area_per_perimeter());

    let shapes: Vec<&dyn Shape> = vec![&c1, &c2, &r1, &r2];

    for shape in shapes {
        println!("{}, {}", shape.area_per_perimeter(), area_per_perimeter(shape));
    }
}

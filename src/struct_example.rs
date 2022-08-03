pub fn customer() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    // Create struct
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };

    // Change a value
    bob.address = String::from("505 Main St");
    println!("Address : {}", bob.address);
}

pub fn shape() {
    /*
        struct Rectangle<T, U> {
          length: T,
          height: U,
      }
      let rec = Rectangle {
          length: 4,
          height: 10.5,
      };
    */

    const PI: f32 = 3.141592;

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    // Define rectangle and circle struct
    struct Rectangle {
        length: f32,
        width: f32,
    }
    struct Circle {
        length: f32,
        width: f32,
    }

    // Implement the trait for rectangle
    impl Shape for Rectangle {
        // Constructor
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width };
        }

        // self allows us to refer to parameters for this struct
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    // Implement the trait for circle
    impl Shape for Circle {
        // Constructor
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    // Create circle and rectangle with Shape
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rec Area : {}", rec.area());
    println!("Circ Area : {}", circ.area());
}

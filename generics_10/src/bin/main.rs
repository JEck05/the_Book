
use std::ops::Add;


struct Point<T>{
    x: T,
    y: T
}
// Example of implementing an add function
// This adds two numbers without taking ownership hence the "&Point<T>"
impl<T: Add<Output = T> + Copy> Add<&Point<T>> for &Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: &Point<T>) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
// Example of implementing a function non-specific to the type of generics
// i.e. runs on all "valid" types
impl<T> Point<T>{
    fn x(&self) -> &T{
        &self.x
    }
}
// Example of a function specific to a certain generic in this case f32
impl Point<f32>{
    fn distance_from_origin(&self) -> f32{
        // distance formula x_1 = 0, y_1 = 0
        // sqrt{ (x_2 - x_1) + (y_2 - y_1) }
        (self.x().powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(&number_list);

    println!("The largest number is {largest}");

    let number_list = vec!['c','d','f'];

    let largest = get_largest(&number_list);

    println!("The largest number is {}", largest);


    let origin = Point{ x: 1.1, y: 2.2 };
    let offset: Point<f32> = Point{ x: 2.2,y: 4.3 };

    let new_point = &origin + &offset;

    println!("x is {} y is {}", origin.x(), origin.y);

    println!("x is {} y is {}", offset.x(), offset.y);

    println!("x is {} y is {}", new_point.x(), new_point.y);

    println!("distance from origin is {}", offset.distance_from_origin());
}
// Example of a generic function that takes in any struct that implement the PartialOrd trait
// returns an immutable reference to the largest number 
fn get_largest<T: PartialOrd>(data_structure: &[T]) -> &T{
    let mut largest = &data_structure[0];

    for number in data_structure {
        if number > largest {
            largest = number;
        }
    }

    largest
}
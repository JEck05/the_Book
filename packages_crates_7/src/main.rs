pub mod garden;

use garden::vegetable::Asparagus;
fn main() {
    garden::main();
    let plant = Asparagus{};
    println!("Im growing {plant:?}");
}
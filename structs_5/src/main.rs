#[derive(Debug)]
struct Rectangle{
    height:u32,
    width: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self{
        Self{
            width:size,
            height:size
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self)-> bool{
        self.width > 0
    }
    fn can_hold(&self, other :&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let rect1 = Rectangle{
        height: 30,
        width: 50,
    };
    let rect2 = Rectangle{
        height: 10,
        width : 40
    };
    let rect3 = Rectangle::square(40);
    if rect1.width() {
        println!(
            "{rect1:?} Area: {} Rect2.Can_Fit:{} Rect3.Can_Fit:{}",
            rect1.area(),
            rect1.can_hold(&rect2),
            rect1.can_hold(&rect3)
        );
    }
}

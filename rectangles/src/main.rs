#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let scale = 3;
    let rect = Rectangle {
        width: dbg!(3 * scale),
        height: 4,
    };

    println!("the rect is: {rect:#?}");
    dbg!(&rect);

    let area: u32 = rect.area();
    println!("The area of the rectangle is {} square pixels.", area);
}

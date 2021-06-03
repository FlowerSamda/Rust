fn main() {

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("The area of the rectangle is {} square pixels.",
    area(&rect1));

    println!("rect1 is {:#?}",  rect1.area()); // 메소드 사용
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("square pixels = {}", sq.area());

}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
// 메소드 정의하기! 완전 편하다!
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    // 연관함수 (static method 같은데 좋다! 미쳤다!)
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}

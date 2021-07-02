// 트레잇 객체를 통해 공통된 동작들에 걸친 추상화를 가능케 함

// 트레잇 객체를 사용하면 트레잇 바운드와 제네릭 타입 파라미터를 사용하는 구조체를 정의하는 것과 다름!
// -> 제네릭 타입 파라미터는 한번에 하나의 구체 타입으로만 대입될 수 있는 반면,
//    트레잇 객체를 사용하면 런타임에 "여러 구체 타입"을 트레잇 객체에 대해 넣을 수 있다.

// -> T 하나로만 되는게 아니라, 여러 구체 타입을 사용할 수 있다는 얘기!

// 공통된 동작을 위한 트레잇 정의하기
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,    // 트레잇 객체를 취하는 벡터 정의
}
/*
//  만약 여기서 components: Vec<T> 였다면,
// 실행 중에 오직 한개의 타입(T에 할당 되는 것)만 사용가능해짐
// 즉, 컴파일 타임에 "단형성화" 되어버려 여러 타입을 사용할 수 없음.
// -> 물론, 하나의 타입만 사용할 때는 제네릭 타입을 하는게 맞다!
*/
// 각 component마다, draw 메소드를 실행시키는 메소드 run 선언
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


// 트레잇 구현하기
// draw는 공통적으로 구현하는 메소드고, Button, Body, SelectBox 등 여러 객체들이 있을 거임
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // Draw 트레잇을 상속했기에, Draw의 세부 구현 등을 적어놓음
    }
}
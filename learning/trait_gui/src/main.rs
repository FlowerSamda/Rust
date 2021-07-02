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



// 만약 SelectBox라는 구조체를 만들려고 할 시,
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 구현 내용
    }


// main 함수에서 돌리기
// components라는 gui 구현할 목록에 SelectBox, Button이 들어감 다른 타입인데도 한 벡터에!
}
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// 결론

// 트레잇 객체를 사용하면, "동일한 트레잇을 구현하는 서로 다른 타입"들의 값을 저장할 수 있다!
// -> 컴파일 시 구체적인 타입이 아니라 응답하는 메시지만을 고려하는 개념!
//    "오리처럼 뒤뚱거리고 꽥꽥 울면 그건 오리임에 틀림없다!"라는 개념과 유사


// 트레잇 객체를 사용하면, 제네릭 타입처럼 컴파일 시점에 모두 구체 타입을 알게되는 "정적 디스패치"대신,
// 런타임에 트레잇 객체 내 존재하는 포인터를 사용하여 어떤 메소드를 호출할 지 알아냄
// -> 이 과정에서, 런타임 비용이 발생하고, 메소드의 코드를 인라인화하는 것을 막아버려 몇몇 최적화 수행 불가


// 트레잇 객체에 대하여 객체 안전성이 요구된다!
// 객체-안전(object-safe)한 트레잇만을 트레잇 객체로 만들 수 있음
// 몇가지 규칙이 있지만, 실전에서는 "2개의 규칙"만 관련되어 있음. 이를 충족하면, 해당 트레잇은 객체안전

// 객체 안전 규칙
// 1. 반환값의 타입이 Self가 아님
// 2. 제네릭 타입 매개변수가 없음

// 설명
// 1. Self 키워드는 우리가 트레잇 혹은 메소드를 구현하고 있는 타입의 별칭.
// 트레잇 객체가 반드시 객체-안전해야하는 이유는 일단 트레잇 객체를 사용시 러스트 컴파일러가 트레잇에
// 구현된 구체적 타입을 알 수 없기 때문
// -> 만약 트레잇 메소드가 고정된 Self타입을 반환하는데, 트레잇 객체는 Self의 정확한 타입을 잊었다면
//    메소드가 원래 구체 타입을 사용할 수 있는 방법 X
// 2. 제네릭 타입 파라미터도 마찬가지로, 그 구체 타입들은 해당 트레잇을 구현하는 타입의 일부가 된다!
//    트레잇을 사용하여 해당 타입을 잊게 되면(트레잇 혹은 메소드를 구현하는 타입이 다시 반환되면),
//    제네릭 타입 파라미터를 채울 타입을 알 수 없음.

// 객체-안전하지 않은 트레잇의 예
// -> Clone 트레잇

// pub trait Clone {
//    fn clone(&self) -> Self;
//}

// 예를 들어, String 타입은 Clone을 구현하고, String 인스턴스에 대하여 clone 메소드를 호출하면
// String의 인스턴스를 반환받을 수 있음! clone 선언은 그냥 자신을 구현하고 있는 타입을 반환하기에,
// Self가 무슨 타입인지 반환할 필요도 없음. 이 과정에서 원래 타입이 "잊히는 것"
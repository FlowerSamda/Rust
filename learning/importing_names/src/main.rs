pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules () {}
        }
    }
}
// use 키워드는 우리가 명시한 것만 스코프 내로 가져옴 (import-from 문이랑 동일!)
use a::series::of::nested_modules;


enum TrafficLight {
    Red,
    Yellow,
    Green,
}
// 열거형 또한 모듈과 비슷하게, 열것값을 use로 가져올 수 있음. 여러개는 중괄호 사용
use TrafficLight::{Red, Yellow};
// 모든 아이템을 가져오기 위해 *(glob) 문법 사용
use TrafficLight::*;


fn main () {
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

}
//---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

// 포인터: 메모리의 주소 값을 담고 있는 변수에 대한 일반적인 개념.
// -> 어떤 다른 데이터를 참조(가리킴)       in Rust, "&"

// 스마트 포인터: 포인터처럼 작동하지만, 추가적인 메타데이터와 능력들도 가지고 있는 데이터 구조
// -> 참조 카운팅(reference counting) 스마트 포인터 타입이 예시
// -> 소유자 수를 계속 추적, 더이상 소유자가 없으면 데이터를 정리(drop) -> 어떤 데이터에 대한 여러 소유자를 만들 수 있게 함.

// 참조자와 스마트 포인터간의 차이
// 참조자: 데이터를 오직 !빌리기만 함!
// 스마트 포인터: 그들이 가리키고 있는 데이터를 !소유!  ex) String, Vec<T>
// String, Vec<T> 가 스마트 포인터인 이유: 얼마간의 메모리 소유, 이를 다루도록 허용, 용량 등의 메타데이터 소유, String이 언제나 유효한 UTF-8일 것임을 보장하는 등의 추가능력 혹은 보장 소유

//-------------------------------------------------------------------------------------------------------------------------------------------

// 스마트 포인터는 보통 !구조체를 이용하여 구현!되어있으나, 일반적인 구조체와 구분되는 특성이 있음
// -> 스마트 포인터는 Deref와 Drop 트레잇을 구현한다는 것.

// Deref trait: 스마트 포인터 구조체의 인스턴스가 참조자처럼 동작하도록 하여, 참조자나 스마트 포인터 둘 중 하나와 함께 작동하는 코드를 작성하게 해줌.
// Drop trait: 스마트 포인터의 인스턴스가 스코프 밖으로 벗어났을 때 실행되는 코드를 커스터마이징 가능하도록 해줌.

/* 표준 라이브러리 내의 가장 흔한 스마트 포인터:
**
** 1. 값을 Heap에 할당하기 위한 Box<T>
** 2. 복수개의 소유권을 가능하게 하는 참조 카운팅 타입인 Rc<T>
** 3. 빌림 규칙을 컴파일 타임 대신 런타임에 강제하는 타입 RefCell<T>를 통해 접근 가능한 Ref<T>, RefMut<T>
*/

//-------------------------------------------------------------------------------------------------------------------------------------------

// Box<T>

/*
** Box<T>는 힙에 있는 데이터를 가리키고 알려진 크기를 갖는다
** Box<T>는 데이터를 스택이 아닌 힙에 저장할 수 있게 해줌. -> 스택엔 힙 데이터를 가리키는 포인터만 남음
** 박스는 스택 대신 힙에 데이터를 저장한다는 점 외에는 성능적인 오버헤드가 없음! 하지만 추가기능도 없음..
*/

// Box<T>를 쓰게 될 상황
/*
** 1. 컴파일 타임에 크기를 알 수 없는 타입을 갖고 있고, 정확한 사이즈를 알 필요가 있는 맥락 안에서 해당 타입 값을 이용하고 싶을 때
** 2. 커다란 데이터를 가지고 있고, 소유권을 옮기고 싶지만, 그렇게 했을 때 데이터가 복사되지 않을 것이라고 보장하기를 원할 때
** 3. 어떤 값을 소유하고 이 값의 구체화된 타입을 알고 있기보다는, 특정 트레잇을 구현한 타입이라는 점만 신경쓰고 싶을 때
*/

//-------------------------------------------------------------------------------------------------------------------------------------------

// Box<T>는 재귀적 타입을 가능하게 한다!

/* 컴파일 타임에서, 러스트는 어떤 타입이 얼마나 많은 공간을 차지하는지를 알 필요가 있음.
** 다만, !재귀적 타입!(recursive type, 어떤 값이 그 일부로서 동일한 타입의 다른 값을 가질 수 있는 것)은 컴파일 타임에 크기를 알 수 없는 타입
** 이러한 값의 내포가 이론적으로는 !무한하게! 계속될 수 있으므로, 러스트는 재귀적 타입의 값이 얼마만큼의 공간을 필요로 하는지 알지 못함.
** -> 여기서 박스는 알려진 크기를 갖고 있기에, 재귀적 타입 정의 내에 박스를 넣음으로써 이를 쓸 수 있음!

** 재귀적 타입의 예시인 Cons list는 Lisp 프로그래밍 언어 및 그의 파생 언어들로부터 유래된 데이터 구조
** Lisp에서, cons(생성함수 construct fnuction의 줄임말) 함수는 두 개의 인자를 받아 새로운 한 쌍을 생성
** -> 이 인자는 보통 단일 값과 또 다른 쌍이고, 이러한 쌍들을 담고 있는 쌍들이 리스트를 형성함.

** 일반적인 함수형 프로그래밍 용어에서, to cons x onto y 
** -> 요소 x를 새로운 컨테이너에 집어넣고, 그다음 컨테이너 y를 넣는 식으로 새로운 컨테이너 인스턴스를 생성하는 것을 의미

*/



fn main() {

    // Box<T>를 사용하여 힙에 데이터를 저장하기
    // 여기서 int32 5는 원래는 스택에 있지만, 힙으로 옮겨감
    let b = Box::new(5);
    println!("b = {}", b);

    // List에서 i32, Box<List>라고 처음의 크기를 이미 정하고, 이후 Nil varient는 어떤 값도 할당하지 않음
    // -> 이후의 데이터는 처음 Box보다 더 작으므로, 크기는 i32 + Box<List>선에서 끝 -> 재귀 X
    use List::{Cons, Nil};
    let list = Cons(1,
         Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}

// 재귀적 타입의 한 예인 Cons List 만들어보기
enum List {
    Cons(i32, Box<List>),
    Nil,
}


// 결론
// Box는 단지 간접 힙 할당만을 제공할 뿐 큰 능력은 없다.
// 단, 재귀적 타입의 경우에 큰 도움이 됨!
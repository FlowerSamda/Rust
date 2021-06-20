// RefCell<T>과 내부 가변성 패턴

// 내부 가변성(interior mutability)
// -> 어떤 데이터에 대한 불변 참조자가 있을 때라도 데이터를 변형할 있게 해주는 디자인 패턴
// unsafe함!

// RefCell<T>을 가지고 런타임에 빌림 규칙을 집행하기
// Rc<T>와는 다르게, RefCell<T>타입은 가지고있는 데이터 상에 단일 소유권을 나타냄.

// Box<T>와 같은 타입에 비교해 RefCell<T>의 다른 부분!
// 컴파일 시점이 아닌 런타임 시점에 빌림 규침을 검사함으로써, 우리는 빌림 규칙 준수를 확신하지만
// 러스트 컴파일러는 이를 이해하지 못할 때 사용하기 유용!

// Box<T>, Rc<T> 혹은 RefCell<T>를 선택하는 이유의 요점

/*
** 1. Rc<T>는 동일한 데이터에 대해 복수개의 소유자를 가능하게 함  <-> Box<T>와 RefCell<T>는 단일 소유자만 가짐
** 2. Box<T>는 컴파일 타임에 검사된 !불변 혹은 가변! 빌림을 허용
**   <-> Rc<T>는 오직 컴파일 타임에 검사된 !불변! 빌림만 허용
**   <-> RefCell<T>은 런타임에 검사된 불변 혹은 가변 빌림을 허용
** 3. RefCell<T>이 런타임에 검사된 가변 빌림을 허용하기 때문에, RefCell<T>이 불변이더라도 내부 값을 변경할 수 있음
**   -> 불변값(RefCell<T>) 내부의 값을 변경하는 것을 !내부 가변성 패턴!이라고 함!
*/

    // let x = 5;
    // let y = &mut x;   x는 불변이기때문에, 가변 참조 불가능!




// Rc<T>와 RefCell<T>를 조합하여 가변 데이터의 복수 소유자 만들기

// RefCell<T>을 사용하는 일반적인 방법은 Rc<T>와 함께 조합하는 것!
// Rc<T> -> !불변 접근에 한해! 복수의 소유자를 허용
// -> RefCell<T>을 들고 있는 Rc<T>를 가진다면, 우리가 변경가능하면서 복수의 소유자를 갖는 값을 가질 수 있음!

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),  // Rc< RefCell<i32> >  Rc를 통해 복수의 불변 소유를 할 수 있고, 그 중 RefCell로 변경 가능!
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));  // 인스턴스 생성

    // Rc::clone(&value)에서 clone을 이용한 이유는, a와 value 둘다 내부의 5 값에 대한 소유권을 얻기 위해 clone 사용(Rc가 있으니까!)
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));  // a는 Rc로 감싸져서 여럿에 의해 소유될 수 있음

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));   // Rc::clone(&a)를 통해, &mut 의 능력을 쓰면서도 deepcopy미수행으로 속도 Up
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;  // RC<T>의 역참조 -> RefCell.RefMut 반환(borrow_mut) -> RefMut 역참조 -> 5

    println!("a after = {:?}", a);  // Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b);  // Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c);  // Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
}

// 이런 식으로, 표면상으로는 불변이지만 내부 가변성 접근(RefCell<T> 이용)을 통해서
// 우리가 원할 때 데이터를 변경시킬 수 있음!

// 이는 런타임에 실행되므로 속도와 약간의 트레이드 오프가 있지만, 이를 감당할 수 있을 때 사용!

// 표준 라이브러리에는 내부 가변성을 제공하는 Cell<T>같은 타입이 있는데,
// Cell<T>은 내부 값의 참조자를 주는 대신, 값이 복사되어 Cell<T> 밖으로 나오는 점만 제외하면 비슷
// Mutex<T>는 스레드들을 건너가며 사용해도 안전한 내부 가변성 존재
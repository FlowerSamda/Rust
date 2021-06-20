// 참조 카운팅 스마트 포인터

// 하나의 값이 여러 개의 소유자를 가지는 경우를 위한 타입 Rc<T>
// 참조 카운팅(reference_counting)의 약자로, 참조자의 개수를 추적함

// **  Rc<T> 스마트 포인터는 불변 참조자를 추적! 가변 참조자면 데이터가 오염될 수 있음!**

enum List {
    Cons(i32, Rc<List>),  //List타입 Rc선언, Rc는 List를 가리키는 스마트 포인터
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;


fn main() {
    // 첫번째 Rc 참조자 (a)
    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));  // 참조 카운트를 볼 수 있는 메소드

    // Rc::clone(&a)는 deep copy를 수행하지 않고, 참조 카운팅을 이용함 -> 시간 절약
    let b = Cons(3, Rc::clone(&a));  // 참조자 개수 1 -> 2,  
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));  // 참조자 개수 2 -> 3
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
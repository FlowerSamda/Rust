// 순환 참조

// 말 그대로 순환하며 서로를 참조 -> 메모리 누수로 인해 프로그램 정지 가능
// Rc<T>값을 가지고 있는 RefCell<T> 혹은 내부 가변성 및 참조 카운팅 기능이 있는 타입을 사용할 때 순환을 만들지 않도록 주의!


use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),  // RefCell을 통해 2번째 요소인 List를 변경 가능
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),  // List 인스턴스.tail() -> 인스턴스가 Cons이면 그 다음 아이템(List) 반환
            Nil => None,
        }
    }
}

fn main() {
    // Cons(5, Nil)
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    

    println!("a initial rc count = {}", Rc::strong_count(&a));  // 1
    println!("a next item = {:?}", a.tail());                   // Some(RefCell { value: Nil })

    // Cons(10, Cons(5, Nil))
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));  // 2
    println!("b initial rc count = {}", Rc::strong_count(&b));           // 1
    println!("b next item = {:?}", b.tail());  //Some(RefCell { value: Cons(5, RefCell { value: Nil }) })

    // Some(link) = (a.tail() == Nil)
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);  // a.tail() = ref item 이었으므로 *link --> Some(Nil) 반환
        // Some(Nil)은 borrow_mut()메서드로 &b를 클론  -> b(a(b(a(b(a))))) 쭉쭉쭊~~~
    }
    
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());  -> 오버플로우 발생
}
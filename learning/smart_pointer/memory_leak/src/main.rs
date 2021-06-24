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
    // println!("a next item = {:?}", a.tail()); // -> 오버플로우 발생


    // 메모리 누수를 방지하기 위한 방법 1
    // -> 참조자를 사용하여 !약한 참조! 사용하기
    use std::rc::Rc;

    /*
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    // branch.children을 통해 leaf로 접근할 수 있지만, leaf는 branch로 접근할 수 있는 방법이 없음
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    */

    // 메모리 누수 방지하기 위한 방법 2
    // -> Rc<T> 대신 Weak<T> 사용하여 약한 참조

    use std::rc::Weak;

    // 노드는 부모 노드를 참조할 수 있지만, 그 부모를 소유하진 않음(Weak이기에!)
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,  // RefCell<  Weak<Node>  > 사용
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),  // 아직 부모 없어서 Weak<new>
        children: RefCell::new(vec![]),
    });

    // strong, weak count 의 변화를 시각화 하기!
    println!(
        "leaf strong = {}, weak = {}",  // 1, 0 (Rc::new(Node {~~~~}))
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );


    // Rc<T>.upgrade => Option<Rc<T>> 반환
    // Rc<T>가 버려지지않았다면 Some(Rc<T>) 반환, 버려졌다면 None 반환
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());    // 현재 노부모 => None
    
    {
        // branch에서 children으로 leaf의 참조자를 클론하여 가져옴
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),  // 새로운 Weak<Node>참조자는 차후 생길 부모 노드를 위한 것
            children: RefCell::new(vec![Rc::clone(&leaf)]),  // leaf의 참조자를 자식으로 가짐
        });

        // Rc:downgrade(&Rc<T>)를 통해 Rc<T>인스턴스 내 값을 가리키는 약한 참조를 만들 수 있다!
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);  // leaf.parent의 값은 branch의 약한 참조가 된다!

        println!(
            "branch strong = {}, weak = {}",  // 1, 1 (branch의 Rc::new(Node), downgrade(&branch))
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",    // 2, 0 (let leaf = Rc::new(), clone(&leaf)) &leaf는 Rc<Node>임을 잊지 말자!
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
    // 여기서 let branch의 강한 참조 1개, Rc::downgrade(&branch)의 약한 참조 1개가 풀려버림 -> 0, 0
    // leaf.parent로부터 발생한 약한 참조는 버려지는 것에 아무런 영향도 주지 않음!(짜피 강한참조만 스코프 종료시 메모리 해제에 영향 줌)
    // 참조가 없어지므로 무한참조 해결!!

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Rc<Node>가 유지중인지 확인

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

}
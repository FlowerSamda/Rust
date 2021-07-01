// 공유 상태 동시성

// 메시지 패싱은 동시성을 다루는 좋은 방법이지만, 유일한 수단은 아님!

// Go 언어 슬로건인 "메모리를 공유함으로써 소통하세요"는 무슨 의미일까?

// 공유 메모리 동시성은 복수 소유권과 유사
// mpsc는 단일 소유권과 유사하게, 채널로 값을 송신하고나면 값을 더이상 쓸 수 없음!
// 하지만, 공유 메모리 동시성은 복수 스레드들이 동시에 동일한 메모리 위치를 접근할 수 있음!

// RefCell<T>을 이용하여 복수 소유권을 다룬 것처럼, 복잡성이 배가되는 경우에서도
// 러스트를 활용하면 서로 다른 소유자들의 관리를 올바르도록 훌륭히 유도할 수 있음.



// 공유 메모리를 위한 더 일반적인 동시성의 기초 재료 중 하나인 뮤텍스(mutex)

// 뮤텍스는 "상호 배제(mutual exclusion)"의 줄임말
// 내부에서 뮤텍스는 주어진 시간에 오직 하나의 스레드에만 데이터 접근을 허용

// -> 뮤텍스의 이러한 특성때문에, 뮤텍스 내부의 데이터에 접근하기위하여
// 스레드는 먼저 뮤텍스의 락(lock)얻기를 요청함으로써, 접근을 원한다는 신호를 보내야한다!

//  마이크가 딱 하나만 있는 컨퍼런스 패널 토의와 비슷!



// Mutex<T>는 연관함수 new를 사용하여 만들어짐
// 뮤텍스 내 데이터에 접근하기 위해서는, "lock 메소드"를 사용하여 락을 얻어야함
// *lock의 호출은 다른 스레드가 패닉 상태의 락을 가지고 있을 시 실패할 수 있기에, 패닉 처리 필요

// Mutex<T>는 스마트 포인터임. 더 정확하게는, lock의 호출은 MutexGuard라고 불리우는 스마트 포인터 반환

// MutexGuard
// 1. 내부 데이터를 가리키도록 Deref가 구현되어있음.
// 2. 스코프 밖으로 버성났을 때 자동으로 락을 해제하는 Drop 구현체를 가지고 있음


// 단일 스레드 맥락 내에서 뮤텍스를 사용하는 예
/*
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();  // mutex의 접근을 얻는 lock 메소드
        // lock을 얻은 후에는 그 반환값(num)을 내부의 데이터에 대한 가변 참조자처럼 다룰 수 있음
        *num = 6;
    }

    println!("m = {:?}", m);              // m = Mutex { data: 6, poisoned: false, .. }
    
}
*/



// 여러 스레드들 사이에서 Mutex<T>를 공유하기

// 오류: 1번 스레드에서 이미 클로저 -> lock으로 인해 이동된 Mutex<i32>타입을 2번 스레드에서 사용하려고 해서
/*
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });

    handles.push(handle2);
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
*/

// Rc<T>를 사용해 복수 소유권을 사용한 예
/*
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        // `Rc<Mutex<i32>>` cannot be sent between threads safely 라고 뜸
        // Rc<T>는 스레드를 교차하면서 공유하기에는 안전하지 않음
        // lock처럼 다른 스레드에 의해 카운트가 변경되는 것을 방해할 수 없도록 확실히 하는 동시성 기초 재료가 없음
        // -> 잘못된 카운트 야기 가능 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
*/



// Rc<T>와 비슷하면서도 "스레드-안전한 방식"으로 참조 카운트를 바꾸는 녀석: Arc<T>

// Arc<T>를 이용하는 아토믹(atomic)참조 카운팅
// Artomic -> 원자적으로 참조자를 세는(atomically reference counted) 타입

// 즉, 기초 타입처럼 동작하지만 스레드를 교차하며 공유해도 안전하다는 것만 알면 됨.


// Arc<T>를 사용한 공유 상태 구현
use std::sync::{Mutex, Arc};                        // Mutex와 Arc는 같은 API
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);         // Arc::clone(&counter로 클론
        let handle = thread::spawn(move || {        // counter가 클로저로 이동
            let mut num = counter.lock().unwrap();  // 클로저에서 뮤텍스(counter)로 이동

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


// 스레드 안전한 방식으로 참조 카운팅하는 것은 속도와 트레이드오프가 있음!
// 따라서, 단일 스레드로 돌리는 등의 작업에서는 굳이 Arc<T> 타입을 쓸 필요가 없음.


// RefCell<T>/Rc<T>와 Mutex<T>/Arc<T> 간의 유사성

// Mutex<T>가 Cell 가족이 그러하듯 내부 가변성을 제공함
// -> 예에서 counter는 불변이지만, 내부 값에 대한 참조자를 가지고 올 수 있었음 (let mut num = counter.lock().unwrap()

// +
// Rc<T> 내부 값을 변경하기 위하여, Rc<RefCell<T>>를 이용했던 것과 유사하게,
// Mutex 내부 값을 변경하기 위하여, Mutex<Arc<T>>를 이용한다!

// -
// 두 RC<T> 타입이 서로를 참조하면서 메모리 누수를 야기하는 순환 참조자를 만들 위험성이 있듯이,
// Mutex<T>는 데드락을 생성할 위험성이 따라옴.



// 데드락 발생 예시
// 어떤 연산이 두개의 리소스에대한 락을 얻을 필요가 있고, 두 개의 스레드가 하나씩의 락을 얻는다면,
// 서로가 서로를 영원히 기다리는 식으로 데드락 발생. (2개가 필요한데 하나씩 서로가 갖고있으니까!)
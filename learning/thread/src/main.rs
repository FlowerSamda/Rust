

    // 대부분의 요즘 운영체제에서, 실행되는 프로그램의 코드는 프로세스 내에서 실행되고,
    // 운영체제는 한번에 여러개의 프로세스들을 관리함
    // -> 프로그램 내에서도 동시에 실행되는 독립적인 부분들을 가질 수 있음!
    // -> "스레드"

    //------------------------------- 스레드의 위험성 ---------------------------------

    // 스레드는 다른 스레드 상에서 실행될 코드 조각들의 실행 순서에 대한 내재적인 보장이 없음
    // 1. 경쟁 조건(race condition)
    // 여러 스레드들이 일관성 없는 순서로 데이터 혹은 리소스에 접근
    // 2. 데드록(deadlock)
    // 두 스레드가 서로 상대방 스레드가 가지고 있는 리소스의 사용을 끝내길 기다려서 양쪽 스레드 모두 실행이 막힘
    // 3. 그 외 많은 버그

    // 프로그래밍 언어가 제공하는 스레드: green thread 
    // 그린 스레드를 사용하는 언얻르은 다른 숫자의 운영체제 스레드로 구성된 콘텍스트 내에서 그린스레드를 실행
    // -> 따라서, 그린 스레드 구조는 M:N (M개의 그린 스레드가 N개의 시스템 스레드에 대응, M=N일 필요 없음)


    // 스레드와 논 스레드 각각의 구조는 고유한 장점과 트레이드 오프를 가지고 있음
    // 러스트에서의 스레드 구현으로 인한 트레이드 오프는 !런타임 지원!이다.
    // 여기서의 런타임은, 언어에 의해 모든 바이너리 내에 포함되는 코드를 의미

    // 모든 어셈블리 아닌 언얻르은 어느 정도 크기의 런타임 코드를 가지게 됨
    // 런타임이 작을 수록, 더 적은 기능을 갖지만 더 작아진 바이너리로 인한 장점을 가짐
    // -> 큰 콘텍스트 내에서 다른 언어들과 조합하기 쉬워진다는 점

    // 그린 스레드 M:N 구조는 스레드를 관리하기 위하여 더 큰 언어 런타임이 필요함
    // 따라서, 러스트 표준 라이브러리는 그린 스레드가 아니라 1:1 스레드 구현만을 제공
    // -> 물론, 더 많은 제어권과 콘텍스트 교환의 더 저렴한 비용 등을 위해 오버헤드를 노출한 크레이트도 존재



    // 러스트 표준 라이브러리 에서의 구현



// 메인 스레드에서 무언가를 출력하는 동안 다른 것을 출력하는 새로운 스레드
/*
use std::thread;
use std::time::Duration;
fn main() {
    
    thread::spawn(|| {
        for i in 1..10000 {
            println!("Hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // 여기서 문제점이 발생. main thread가 끝나면 spawn된 스레드가 조기 종료됨
    // -> 스레드들이 실행되는 순서에 대한 보장이 없음 -> join 메서드의 필요성
    for i in 1..5000 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}
*/



// join 핸들을 사용하여 모든 스레드들이 끝날때까지 기다리기
/*
use std::thread;
use std::time::Duration;

fn main() {
    // thread::spawn -> JoinHandle<T>
    // JoinHandle: 이것이 가지고 있는 join 메소드를 호출했을 때, 그 스레드가 끝날때까지 기다리는 소유된 값
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle의 타입은 JoinHandle이고, 여기서 join 메서드를 호출!
    // -> main 스레드가 끝나도 handle의 스레드가 계속 됨
    handle.join().unwrap();
    // 핸들에 대해 join을 호출하는 것은 핸들에 대한 스레드가 종료될 때까지 현재 실행중인 스레드를 블록
    // 블록(block)은 그 스레드의 작업을 수행하거나 종료되는 것이 방지된다는 의미
}
*/



// join 메서드 위치의 역할
/*
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();  // main 스레드 전에 join 메서드 선언
    // -> 생성된 스레드가 종료된 후 main 실행 -> 디테일이 중요!

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
*/


// 스레드에 move 클로저 사용하기
// move 클로저는 스레드의 데이터를 다른 스레드 내에서 사용하도록 해주는 기능을 함!
// 클로저의 파라미터 목록 앞에 move 키워드를 이요하여 클로저가 그 환경에서 사용하는 값의 소유권을
// 강제로 갖게 한다고 언급했는데, 이 기술은 "한 스레드에서 다른 스레드로 이전하기 위해 새로운
// 스레드를 생성할 때" 특히 유용함!

// 이전의 thread::spawn에 넘기는 클로저는 아무런 인자도 갖지 않는다는 점을 주목해야함!
// 생성된 스레드의 코드 내에서는 메인 스레드로부터 "어떤 데이터도 이용하고 있지 않음"!
// 메인 스레드로부터의 데이터를 생성된 스레드 내에서 사용하기 위해서는
// 생성된 스레드의 클로저가 필요로하는 값을 "캡쳐" 해야함!
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    /* 오류 예시 1
    let handle = thread::spawn(|| {
        // 클로저는 v를 사용하여 v는 캡쳐되어 클로저의 환경일부가 됨
        // 그래서, thread::spawn이 이 클로저를 새로운 스레드 내에서 실행하기에
        // v는 새로운 스레드에서 접근 가능해야하지만, 
        println!("Here's a vector: {:?}", v);  // {:?}는 참조자를 필요로 함. 하지만, "얼마나 참조자가 유효한지 모름"
    });
    */

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v); // v를 캡쳐(소유권 획득)하지 않았기에, 오류!
    });

    handle.join().unwrap();
}

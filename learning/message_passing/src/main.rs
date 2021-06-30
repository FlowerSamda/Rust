// 메시지 패싱(message passing)

// 안전한 동시성을 보장하는 인기 상승 중인 접근법
// -> "스레드 혹은 엑터들이 데이터를 담고있는 메시지를 서로 주고받는 것!"

// 러스트에서  메시지 보내기 방식의 동시성을 달성하기 위해 갖춘 한가지 주요 도구는 "채널"
// 채널은 강의 상류와 하류와 같음

// 프로그래밍에서의 채널은 둘로 나뉘어짐  |송신자| --- |수신자|
// 보내고자하는 데이터와 함께 송신자의 메소드 호출 -> 다른 곳에서는 도달한 메시지에 대한 수신 종료 검사
// 송신자 혹은 수신자가 드롭되면 "채널이 닫혔다"(closed)라고 말함



// 단순한 채널

// mpsc 설명

// mpsc: 복수 생성자, 단수 소비자를 의미 (multiple producer, single consumer)
// -> 한 채널이 값을 생성하는 복수개의 송신 단말을 가질 수 있지만, 값을 소비하는 수신하는 단말은 한개만 가능\

// mpsc::channel 함수는 튜플을 반환
// (송신자, 수신자)  == (transmitter, receiver) == (tx, rx)

// rx.recv() -> Result<T, E>반환           (recv 는 receive의 준말)
// -> 가능한 메시지가 있다면(Ok) 이를 처리하고, 메시지를 기다리는 동안, 다른 작업을 하는 루프를 만들 수 있음


// 송신자에서 채널로 내려보낸 후, 그 내보낸 데이터에 있는 변수를 사용하려고 하면 어떨까?
// -> 값을 다시 사용하기 전에 스레드에서 수정되거나 버려질 수 있어서 에러 가능!



// 송신자를 생성된 스레드로 이동시키고, 하나의 스트링을 전송하여 메인스레드와 통신하는 채널

fn main() {
    /*
    use std::sync::mpsc;
    use std::thread;


    let (tx, rx) = mpsc::channel();

    thread::spawn(move|| {
        let val = String::from("hi!");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got:{}", received);     // Got:hi! 출력
    */



    // val을 채널로 내려보낸 뒤의 이에 대한 사용 시도
    /*
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();       // send메서드에서 이미 val에 대한 소유권을 가져감
        println!("val is {}", val);  // 채널로 내려보낸 뒤(소유권 X) 이에 대한 사용 시도
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    */



    // 복수의 값들을 보내고 수신자가 기다리는지 보기
    /*
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        // 여러번 내보내는 부분
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx.recv()를 명시하지 않고, rx를 반복자처럼 다루고있다!
    // 어떠한 멈춤 혹은 지연 코드를 넣지 않았기에, 메인스레드가 값을 자동으로 기다림을 알 수 있다!
    for received in rx {
        println!("Got: {}", received);
    }
    */



    // 송신자를 복제하여 여러 생성자 만들기
    // mpsc의 의미처럼, 여러개의 송신자를 만들어보기!
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    // mpsc::Sender::clone(&tx) ->  다른 스레드가 사용할 수 있는 새로운 송신 핸들 제공
    // -> tx와 tx1(복수의 송신자)는 rx(단일 수신자)에게 보냄
    let tx1 = mpsc::Sender::clone(&tx);
    // 스레드 1번 tx1 사용
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
    
        for val in vals {
            tx1.send(val).unwrap();                // tx1
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 스레드 2번 tx 사용
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
    
        for val in vals {
            tx.send(val).unwrap();                 // tx
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
    

}



// 결론: mpsc의 channel메서드, Sender::clone메서드를 사용하면
// 괜찮은 동시성을 구현할 수 있다!
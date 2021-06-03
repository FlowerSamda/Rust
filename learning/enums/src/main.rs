#![allow(unused)]
fn main() {

    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);

    enum IpAddrKind {
        V4(u8, u8, u8, u8),  // 이름 열거형의 variants라고 한다!
        V6(String),          // V4, V6의 예시처럼, 각 varient는 다른 타입과 다른 양의 연관된 데이터를 가질 수 있다!
    }
    
    // 각 varient들은 다른 데이터 타입을 가질 수 있고, 모든 varients가 Message타입으로 그룹화되는게 enum의 장점!
    #[derive(Debug)]
    enum Message {
        Quit,                       // 유닛 구조체
        Move { x: i32, y: i32 },    // struct
        Write(String),              // String을 가지는 튜플 구조체
        ChangeColor(i32, i32, i32), // 3개의 i32를 가지는 튜플 구조체
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", &self)
        }
    }

    fn route(ip_type: IpAddrKind) {
    
    }

    let m = Message::Write(String::from("hello"));
    m.call();
/////////////////////////////////////////////////////////////////////////////////////////////
    
    //Option 열거형, T는 제너릭 타입(Some variant가 어떤 타입의 데이터라도 가질 수 있음을 의미)
    enum Option<T> {
        Some(T),
        None,
    }


    let some_number = Some(5);
    let some_string = Some("a string");
    // Some이 아닌 None을 사용할 때에는, Option<T>가 어떤 타입을 가질 지 알려줄 필요가 있음. 
    // Compiler는 None만 보고는 Some variant가 어떤 타입인지 추론할 수 없음.
    let absent_number: std::option::Option<i32> = None;  // Option<i32> = None하면 왜 main::Option으로 뜰까?
    // Option<T>와 T는 다른 타입이라, 컴파일러는 Option<T> 값을 명확하게 유효한 값처럼 사용하지 못하게 함.
    /*
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x+y;

    -> error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
    not satisfied  
    즉, Option<i8>과 i8을 어떻게 더해야 하는지 모른다는 것을 의미 (둘은 다른 타입이기에!)
    */
    
}



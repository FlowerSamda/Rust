fn main() {
    // cargo.toml에 이렇게 하면, 되감기를 그만두고 바로 종료 가능
    //[profile.release]
    //panic = 'abort'  

    /*
    panic!("crash and burn");

    RUST_BACKTRACE=1 cargo run 입력시 백트레이스 시작
    let v = vec![1, 2, 3];
    v[99]; // 인덱스 범위를 넘어선 값에 접근 -> "버퍼 오버리드" 방지위해  패닉
    */
    

    //  Result와 함께하는 복구 가능한 에러

    /*       \\ Result 열거형은 다음과 같이 Ok, Err라는 두 variant를 가짐//
    enum Result<T, E> {
        OK(T),
        Err(E),
    }
    */

    use std::fs::File;
    // File::open은 Result 열것값 가짐

    // let f:u32 = File::open("hello.txt");  // 일부러 u32라는 잘못된 타입 명시를 하면, 컴파일러가 타입을 알려줌!
    //-> note: found enum `Result<File, std::io::Error>`

    let f = File::open("hello.txt");

    // 단순히 Err시 panic을 발생하는 방법.

    // f 는, f가 OK타입이면 file, Err타입이면 panic!
    /* 
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
    */

    // Err 중 파일이 없어서 실패한 것이라면, 새로운 파일을 만들어 핸들 반환하기

    /*
    use std::io::ErrorKind;
    
    let f = match f {
        Ok(file) => file,
        // ref를 사용함으로서, error의 소유권은 이동하지 않고 값만 참조함 &사용 안하는 이유는 18장(대충 참조자를 매치하고 값을 제공하는건 &, 값을 매치하고 참조자를 제공하는건 ref)
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                // 파일 생성 실패시
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                }
            }
        },
        // 파일 찾기 에러가 아닌 애들 잡기
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
    */


    //*** Error가 났을 때 패닉을 위한 숏컷! unwrap과 expect

    // match의 사용은 충분히 잘 작동하지만, 살짝 장황하기도 하고 의도를 항상 잘 전달하진 않음.
    // 그래서, Result<T, E> 타입의 헬퍼메소드 중 하나인 unwrap!

    // unwrap()메소드는 Result 열거자가 Ok라면, unwrap은 ok내의 값 반환, Err 열거자라면 panic!매크로 호출
    //let f = File::open("hello.txt").unwrap();  //panic 호출이 됨!

    // expect메소드는 unwrap과 유사한데, 우리가 panic! 에러 메시지를 선택할 수 있게 해줌 (내용 설명 가능!)
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");  // panic 시 Failed ~~ 반환


    //*** Error 전파하기!
    // : 에러가 발생할 수도 있는 함수에서 에러가 발생 시, 이 함수말고 에러를 호출하는 코드 쪽으로
    //   에러를 반환하여 그 쪽에서 어떻게 할 지 결정하도록 만드는 것 => 호출하는 코드쪽에 더 많은 제어권 부여
    
    /*
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        // 함수의 반환타입은 Result<T, E>에서 T는 구체적 타입인 String, Error는 io::Error로 있음.
        // 문제없이 성공하면, 함수를 호출한 코드는 String을 받고, 문제가 있다면 더 많은 정보를 담고있는 io:Error를 반환
        // io::Error를 선택한 이유는, File::open()과 Read::read_to_string이 모두 io::Error 타입 반환
        let f = File::open("hello.txt");
    
        // 문제 없을 시 let mut f = file로 파일핸들을 f에 저장, 있다면 return으로 read_~함수 끝냄 => io:Error반환!
        let mut f = match f {
            Ok(file) => file,
            // (왜 return? => 이 함수를 일찍 끝내고 File::open()에서의 에러값을 마치 이 함수의 에러값인것처럼 호출하는 쪽의 코드에게 전달)
            Err(e) => return Err(e),  
        };
    
        let mut s = String::new();
        // Read::read_to_strig
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),  // 값이 OK이면, 이걸 OK(s)로 저장을 해서 반환
            Err(e) => Err(e),  //return을 안한 이유는 짜피 마지막 표현식이라서 ㅎ;;
        }
        //결론적으로, 이 함수는 2개의 과정이 있음에도 불구하고, 무조건 OK값 아니면 io::Error를 반환!
    }
    */

    // 에러를 전파하기 위한 숏컷: ?
    // enum Result 뒤에 ? 를 붙이면, 값이 OK시에는 Ok 내의 값이 얻어지고 계속 진행됨
    // 값이 Err라면, return 키워드를 사용하여 전체 함수로부터 Err값이 반환 (return Err(e))
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;  // ? 사용:
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    // 심지어 더 줄일 수도 있다!
    /*
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;  //이렇게 그냥 바로 메서드를 붙일 수도 있음;

        Ok(s)
    }
    */


    // 추가! ?는 result를 반환하는 함수에서만 사용 가능!!
    // main함수는 반환타입이 ()(빈 유닛, 아무고토안함)이기 때문에, main함수 내에서 쓰면 오류남!
    // 그래서, Result를 반환하지 않는 함수 내에서는 match나 Result에서 제공하는 expect, unwrap 등 사용!

    // 보통 패닉을 발생시키는 것보다, Result를 반환하는 것이 좋다!
    // 그리고, 처음부터 빡세게 할 필요없이, unwrap이나 expect 메소드로 우선 구현하고, 나중에 보고 고치기!

    // 내가 컴파일러보다 더 많은 정보가 있을 때, unwrap을 호출하는게 적절할 수 있음
    
    use std::net::IpAddr;

    let home = "127.0.0.1".parse::<IpAddr>();  // 여기서 127.0.0.1은 무조건 Ok값인데, 컴파일러는 에러가 나올 수 있다고 생각함
    //그래서,  .unwrap()을 붙이면, Result값은 Err 열거자가 나올 수 있는 가능성이 있어보이게 만듦

    // 위의 방법들에도 불구하고, panic을 호출하는 것은 다음과 같은 경우에 함.
    /*
    1) 이 나쁜 상태란 것이 가끔 벌어질 것으로 예상되는 무언가가 아닙니다. 
    2) 그 시점 이후의 코드는 이 나쁜 상태에 있지 않아야만 할 필요가 있습니다.  ex) 보안문제(인덱스 범위를 벗어나는 접근 등)
    3) 여러분이 사용하고 있는 타입 내에 이 정보를 집어 넣을만한 뾰족한 수가 없습니다.
     */

    // 생성부터 오류 가능성을 피하고, panic을 적절하게 사용하여 반복을 줄인 것 (panic안쓰면 계속 확인해야해서 성능도 안좋음)
    pub struct Guess {
        value: u32,
    }
    impl Guess {
        //Guess::new() 정의 ->Guess객체 반환 Guess{value}
        pub fn new(value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }
    
            Guess {
                value  // 변수명과 필드명이 같을 때 간단하게 필드 초기화하기!
            }
        }
    
        pub fn value(&self) -> u32 {
            self.value
        }
    }
}

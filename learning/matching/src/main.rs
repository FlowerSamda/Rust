fn main() {
    //////////////////////////match

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        California,
        Washington
    }

    //열거형과 열거형의 variant를 패턴으로서 사용하는 match 표현식
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");  // 이런 식으로 중괄호를 사용할 수 있다
                1
            },  // 갈래들(arms)             //'패턴 => 코드' 형태
            Coin::Nickel => 5,
            Coin::Dime => 10,
            // 바인딩을 사용하는 부분: Quarter(UsState::Alaska)를 인자로 전달했을 시,
            // coin은 Coin::Qu~~~~Alaska)임 이는 각각 매치 갈래들과 비교하지만, Coin::Quarter(state)
            //로 도달할 때까지 아무것도 매치 X 이 시점에서 state에 대한 바인딩은 UsState::Alaska임.
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);  // state바인딩 == UsState::Alaska
                25
            },
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Option<T>를 이용하는 매칭 https://rinthel.github.io/rust-lang-book-ko/ch06-02-match.html
    
    fn plus_one(x: Option<i32>) -> Option<i32> {
        
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
}


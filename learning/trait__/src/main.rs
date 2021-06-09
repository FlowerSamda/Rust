// trait은 인터페이스(Solidity의 그 인터페이스!!)와 비슷함(다른 점도 있공)
// 공통적으로 갖는 동작에 대하여 추상화하도록 해줌! 


// trait 정의하기!
// 어떤 타입의 동작은 우리가 해당 타입에서 호출할 수 있는 메소드들로 구성되어있다고 가정했을 때,
// 만일 우리가 서로 다른 타입에 대해 모두 동일한 메소드를 호출할 수 있다면, 이는 이 타입들이 동일한 동작을 !공유!한다는 것! (=interface)
// !트레잇(trait)의 정의!
// -> 어떠한 목적을 달성하기 위해 필요한 동작의 집합을 정의하기 위해 메소드 시그니쳐들을 함께 묶는 것


/*
    trait 예시
    다양한 종류와 양의 텍스트를 갖는 여러 가지의 구조체를 가지고 있다고 칩시다: NewsArticle 구조체는 세계의 특정한 곳에서 줄지어 들어오는 뉴스 이야기를 들고 있고, Tweet은 최대 140글자의 콘텐츠와 함께 해당 트윗이 리트윗인지 혹은 다른 트윗에 대한 답변인지와 같은 메타데이터를 가지고 있습니다.

    우리는 NewsArticle 혹은 Tweet 인스턴스에 저장되어 있을 데이터에 대한 종합 정리를 보여줄 수 있는 미디어 종합기 라이브러리를 만들고 싶어 합니다. 각각의 구조체들이 가질 필요가 있는 동작은 정리해주기가 되어야 하며, 그래서 각 인스턴스 상에서 summary 메소드를 호출함으로써 해당 정리를 얻어낼 수 있어야 한다는 것입니다. Listing 10-11은 이러한 개념을 표현한 Summarizable 트레잇의 정의를 나타냅니다:

*/

// 트레잇 선언
pub trait Summarizable {
    // 이 트레잇을 구현하는 타입들이 가질 필요가 있는 동작을 묘사한 메소드 시그니처
    fn summary(&self) -> String;  // 정의 대신 ; 삽입

    // -> Summarizable 트레잇을 가지는 어떠한 타입이든, 그에대한 메소드 summary를 정확히 동일한 시그니처로 정의되도록 강제 !! (이거 완전 interface 아니냐??)
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// NewsArticle에 대해 Summarizable을 사용하여 summary 구현
// impl 구현하고자하는 타입 for 메소드명
// 원래는 impl NewsArticle { fn ~~ }
impl Summarizable for NewsArticle {
    // summary(&self) -> String 시그니처 완벽 구현
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// 여기서도 구현!
impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}



fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    // 트레잇을 한 번 구혆면, 트레잇의 일부가 아닌 메소드를 호출했던 것과 동일한 방식으로 인스턴스 상 호출 가능!!
    println!("1 new tweet: {}", tweet.summary());


    // 만약 트레잇이 동일 scope 내에 있지 않고, aggregator라는 크레이트에 대한 것이라면...?

    /*
    extern crate aggregator;

    *** 이런 식으로 가져온다 (상대위치)  !주의! pub 선언 되어있어야함!
    use aggregator::Summarizable;

    struct WeatherForecast {
        high_temp: f64,
        low_temp: f64,
        chance_of_precipitation: f64,
    }

    impl Summarizable for WeatherForecast {
        fn summary(&self) -> String {
            format!("The high will be {}, and the low will be {}. The chance of
            precipitation is {}%.", self.high_temp, self.low_temp,
            self.chance_of_precipitation)
        }
    }
    */    

    // 트레잇 구현과 함께 기억할 한 가지 제한사항!
    // -> trait 혹은 type은 !우리의 크레이트 내의 것일 경우에만 해당 타입에서의 트레잇을 정의할 수 있음!

    // ex) Vec에 대한 Display 트레잇은 구현이 불가능한데, Display와 Vec 모두 표준 라이브러리 내에 정의되어있기 떄문.

    // -> 우리의 aggregator 크레이트 기능의 일부로서, Tweet과 같은 커스텀 타입에 대해 Display와 같은 표준 라이브러리 트레잇을 구현하는 것은 허용됨.
    // -> 동일하게, aggregator 크레이트 내에서 Vec에 대하여 Summarizable을 구현하는 것도 가능 

    // !모두, aggregator가 우리 crate 내에서 정의되어 있기에 가능!

    // 이는 Orphan rule (고아 규칙)이라고 불리는 것의 일부!

    // 간단히 말해, 부모 타입이 존재하지 않기 떄문이라는 뜻
    // 이 규칙이 없다면, 두 크레이트는 동일한 타입에 대해 동일한 트레잇을 구현할 수 있게되어, 두 구현체가 충돌을 일으킬 수 있음

    // ex) Vec에 Display가 있는데 또 쳐만드는 것 등

    // https://www.youtube.com/watch?v=L-03Rc4j_9g


    // 기본 구현
    // 이를 상속한 모든 구현체가 커스텀 동작을 하는 것 말고, 기본 동작을 갖추는 것이 필요할 때!
    // -> 기본 구현(동작)을 유지하거나, 오버라이드하도록 할 수 있음

    pub trait Summarizable_plus {
        fn author_summary(&self) -> String;
        // author_summary를 구현하도록 하고, summary메소드가 이를 호출하는 기본 구현
        fn summary_plus(&self) -> String {
            format!("(Read more from {}...)", self.author_summary())
        }
    }

    //summary_plus의 기본 구현 사용을 위해서는 이렇게 사용
    // impl summary_plus for NewsArticle {}

    impl Summarizable_plus for Tweet {
        fn author_summary(&self) -> String {
            format!("@{}", self.username)
        }
    }

    println!("1 new tweet: {}", tweet.summary_plus());


    // 트레잇 바운드
    // 제네릭 타입 파라미터를 이용하는 트레잇: 제네릭 타입에 제약을 가함
    // -> 제네릭 타입에 대해 트레잇 바운드를 사용했다면, 제네릭 타입을 가진 파라미터가 그 트레잇을 반드시 구현했다는 것을 알려줌

    //notify라는 함수명 뒤에 <T: 트레잇 명>(parameter: T)로 선언
    // ->이는 T타입의 파라미터가 그 트레잇을 구현했다는 것을 의미
    pub fn notify<T: Summarizable>(item: T) {
        println!("Breking news! {}", item.summary());
    }

    // 여러 트레잇을 가진 제네릭 타입에 대한 표시
    // -> fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    // where 절을 이용하면 더 보기 쉬워짐
    /* 
    fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
            U: Clone + Debug 
    {
    }
    */
        
    // 이런 Trait bound를 사용하여, generic에서의 largest함수 고치기!
    fn largest_generic<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            // 오류 지점
           if item > largest {
               //오류 지점
               //`T` might need a bound for `std::cmp::PartialOrd` 오류 발생
               largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_generic(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_generic(&chars);
    println!("The largest char is {}", result);    


    //Copy 트레잇을 이용하지않고, 참조자로 굴러가보도록 해보기!
    
    fn largest_generic_2<T: PartialOrd>(list: &[T]) -> &T {
        
        let mut largest = &list[0];  // largest = &&T[0]

        // iter() ->&T
        for item in list.iter() {
            // 오류 지점
           if item > largest {
               //오류 지점
               //`T` might need a bound for `std::cmp::PartialOrd` 오류 발생
               largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_generic_2(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_generic_2(&chars);
    println!("The largest char is {}", result);    
    

}   

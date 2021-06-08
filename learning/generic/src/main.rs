fn main() {
    
    // Generic: 구체화된 타입이나 다른 속성들에 대하여 추상화된 대리인

    // 함수를 추출하여 중복 없애기
    
    /*
    let numbers = vec![34, 50, 25, 100, 65];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    *** 다른 숫자 리스트도 추출하려고 할 시!
    *** 중복 코드때문에 지루하고 오류도 발생하기 쉬움. 로직 변경시 2번 바꿔야함(똑같으니까!)
    *** 여기서 추상화가 들어가는데, 이 경우는 "어떠한 정수 리스트가 함수 파라미터로 주어졌을 때 동작하는 함수"로 추상화!

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = numbers[0];

    for number in numbers {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    */

    //이렇게 하면, https://rinthel.github.io/rust-lang-book-ko/ch10-00-generics.html 여기서부터
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];
    
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }
    
    
    let numbers = vec![34, 50, 25, 100, 65];
    
    let result = largest(&numbers);
    println!("The largest number is {}", result);
    
    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
    let result = largest(&numbers);
    println!("The largest number is {}", result);


    // i32타입과 char 타입에 제네릭하게 사용하는 함수 만들기
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&chars);
    println!("The largest char is {}", result);

    // 오류가 발생하는 이유는, T가 될 수 있는 모든 타입에 적용되지 않을 것이라는 의미 (순서대로 정렬하는 방법을 아는 타입만 할 것이라는 뜻)
    //  표준 라이브러리에 어떤 타입에 대해 비교 연산이 가능하도록 구현할 수 있는 "트레잇"인 PartialOrd 사용
    /*
    fn largest_generic<T>(list: &[T]) -> T {
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
    */


    // 구조체 내에서 제네릭 데이터 타입 사용하기
    // 2개 이상 타입을 위해서는 <T, U>로 해서 해야함 (int, float을 받을 경우 처럼)
    struct Point<T> {
    x: T,
    y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };


    // 열거형 정의 내에서 제네릭 데이터 타입 사용하기
    // Option<T>는 T타입에 제네릭인 열거형 => 다양한 타입에 사용할 수 있는 추상화된 개념!
    enum Option<T> {
    Some(T),
    None,
    }

    // 이런 식으로 열거형도 2개 이상 타입을 사용 가능!
    enum Result<T, E> {
        OK(T),
        Err(E), 
    }

    
    // 메소드에 사용하기!
    // T를 받는 struct 생성!
    struct PointForMethod<T> {
        x: T,
        y: T,
    }
    // impl 뒤에 <T>가 받는 것을 주목!!
    // struct<T>의 객체 생성, 메소드 생성
    impl<T> PointForMethod<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = PointForMethod {x: 5, y: 10};
    println!("p.x = {}", p.x());

    // 구조체 정의와 다른 제네릭 타입 사용 메소드 예시
    struct PointForAnother<T, U> {
    x: T,
    y: U,
    }

    impl<T, U> PointForAnother<T, U> {
        // 여기서, <V, W>를 제시함으로서 다른 제네릭 타입을 가진 것을 사용 가능!
        // T, W를 선언함으로서 2개가 섞임
        // T, U는 오직 impl에서만 선언됐는데, 이는 구조체 정의와 함께 사용되어서이고, 메소드에 관련된 V, W만이 메소드 부분에 선언
        fn mixup<V, W>(self, other: PointForAnother<V, W>) -> PointForAnother<T, W> {
            PointForAnother {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = PointForAnother { x: 5, y: 10.4 };
    let p2 = PointForAnother { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    // 제네릭 타입을 사용하는 것은 런타임 비용을 전혀 소모하지 않는다!!

    // '단형성화'를 구현하여, 컴파일러는 전체 코드를 읽고 제네릭 정의들을 명시적인 것으로 교체함.

    //ex) Option<T>는 넘겨진 값들을 읽어서, i32, i64 두가지가 있음을 앎
    // -> 제네릭 정의를 명시적인 것들로 교체하여 제네릭 정의를 Option_32, Option_64로 확장
    // -> 결국 컴파일 하면 2개가 있는 것과 같고, 비용이 들지 않는다!




}

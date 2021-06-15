// 반복자: 각 항목을 순회하고 언제 종료될지 결정하는 로직을 담당

fn main() {

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();  // 반복자 생성(그냥 생성만 됨)

    // fn into_iter(self)이기에, 소유권이 넘어가게되어 다시한번 더 v1_iter 사용 불가능
    for val in v1_iter {
        println!("{}", val);  // 1, 2, 3
    }

    // 모든 반복자는 표준 라이브러리에 정의된 Iterator라는 이름의 트레잇을 구현!
    trait Iterator {
        type Item;  // 여기서 type Item은 이 트레잇과 연관 타입을 정의 
    
        fn next(&mut self) -> Option<Self::Item>;  // Item 타입이 반환될 것이라는 얘기 (Option -> Some(val), None 반환 )
    
        // methods with default implementations elided
    }

    // 반복자 어댑터: 반복자를 다른 종류의 반복자로 변경하도록 허용함
    let v1: Vec<i32> = vec![1, 2, 3];
    
    // map: !새로운 반복자를 생성!하기 위해 각 항목에 대해 호출할 클로저를 인자로 받음!
    // map을 사용하면, 기존 반복자를 통해 무한한 가능성이 있는 반복자들을 만들 수 있음!
    v1.iter().map(|x| x + 1);  // 오류 발생(iterators are lazy and do nothing unless consumed)

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // 이터레이터의 collect는 반복자 소비 후 결과값을 수집 데이터 타입으로 모음!
    assert_eq!(v2, vec![2, 3, 4]);


}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));  // Some(&1) -> 벡터 안의 불변참조 값을 줌
    assert_eq!(v1_iter.next(), Some(&2));  // 만약 불변참조가 아니라 소유권을 갖고 소유된
    assert_eq!(v1_iter.next(), Some(&3));  // 값들을 받고 싶다면, into_iter 호출!
    assert_eq!(v1_iter.next(), None);      // 가변참조는 iter_mut 호출!
}



// 환경을 캡쳐하는 클로저 사용하기
    // filter 반복자 어댑터: 반복자로부터 각 항목을 받아 Boolean을 반환하는 클로저를 인자로 받음
    // -> 클로저가 true 반환시, filter에 의해 생성되는 반복자에 포함, 아닐시 제외
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();  // sum 메소드: 반복자 소비가 끝나고 순회가 끝나면 합계 반환

    assert_eq!(total, 6);
}


// 구조체 선언
#[derive(PartialEq, Debug)]  // 값 비교 위한 PartialEq와 Debug 필요 의미
struct Shoe {
    size: u32,
    style: String,
}

// Shoe구조체 타입으로 이루어진 shoes Vec을 반복자 생성(into_iter())후 필터, 콜렉트 해서 반환
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()                     //소유권을 갖는 iterator 반환
        .filter(|s| s.size == shoe_size)  // s.size가 맞으면 shoes 벡터 반환
        .collect()                        //반복자 소비 후 수집, 반환
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}


// Iterator 트레잇으로 나만의 반복자 만들기
struct Counter {
    count: u32,
}

impl Counter {
    fn new () -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;  // 연관 타입: 반복자가 u32값을 반환한다는 것을 의미
    // 변경 가능한 self 받음 -> Option<Self::u32>
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// 표준 라이브러리에 정의된 트레잇 메서드들의 기본 구현 사용해보기 (모두 next 메서드 기능을 사용하기때문에, 사용가능)
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))  // 각 인스턴스끼리 쌍을 이룸. Zip<0, 1>, <1, 2>, ... 4개만 나오는 이유는 하나라도 None이면 None 반환
                                 .map(|(a, b)| a * b)          // a*b를 한 이터레이터 반환
                                 .filter(|x| x % 3 == 0)       // 3으로 나누어 떨어지는 것들만 이터레이터로 반환
                                 .sum();                       // 합
    assert_eq!(18, sum);
}
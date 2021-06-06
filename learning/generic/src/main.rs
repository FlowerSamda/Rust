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







}

// 클로저: 변수에 저장하거나 다른 함수에 인자로 넘길 수 있는 익명 함수\
// 환경을 캡처할 수 잇는 상황을 표현하는 것은 함수 파라미터로서 유용!
use std::thread;
use std::time::Duration;

// 클로저가 환경을 캡처하는 3가지 방식
// 러스트는 클로저가 환경에 값을 어떻게 사용하는지에 근거하여 자동으로 추론
// -> Fn으로 시작해보면, 컴파일러는 클로저 바디에서 무슨 일을 하는지에 근거하여 FnMut, FnOnce가 필요한지 말해줌!!
    // FnOnce: 클로저의 환경( 둘러싼 환경 )에서 캡처한 변수들을 소비 -> Once는 동일한 변수들에 대해 한번이상 소유권을 얻을 수 없음을 의미
    // Fn    : 그 환경으로부터 값들을 불변으로 빌려옴
    // FnMut : 값들을 가변으로 빌려옴 (환경을 변경 가능)

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    // 클로저의 기능 중 하나: 환경을 캡처해서, 클로저가 정의된 스코프의 변수들을 접근할 수 있음!
    let x = 4;
    // x가 equal_to_x의 파라미터가 아니더라도, eqaul_to_x는 자신이 정의된 스코프에 정의된 x 변수 사용 가능!
    let equal_to_x = |z| z==x; // 값을 읽기만하면되기에, Fn으로 불변으로 빌려오기만 함 
    let y = 4;
    assert!(equal_to_x(y));    //통과

    //함수는 이렇게 못함

    //let x = 4;
    //fn equal_to_x(z: i32) -> bool { z == x }  // can't capture dynamic environment in a fn item
    //let y = 4;
    //assert!(equal_to_x(y));   

}

// 실행시간이 2초 걸리는 가상의 계산을 대신하는 함수
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}


/*
expensive_calculation을 여러번 호출하는 함수 -> 중복된 함수를 호출하여 너무 많은 시간이 걸림

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
*/

// 프로그램 한 곳에서 코드를 정의하고, 그 결과가 필요한 곳에서만 코드를 실행하는 것
// -> 클로저의 유스케이스

/*
fn generate_workout(intensity: u32, random_number: u32) {
    // 중복됐던 expensive_calculation 함수 호출을 하나의 변수로 추출 -> 한번만 호출되고, 리턴값 사용
    let expensive_result =
        simulated_expensive_calculation(intensity);   *** 여기서 expensive 호출이 실행(시간 걸림)
    
        *** 결과적으로, 이 아래 모든 코드들이 이 변수를 쓰지 않는 경우에도 기다리게됨
        *** 실제 결과가 필요한 곳에서만 이 코드를 실행하려면??? -> 클로저 사용

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
*/


/*
fn generate_workout(intensity: u32, random_number: u32) {
    // 클로저 정의는 변수에 할당하기위해, = 다음에 옴 ( expensive_closure = |num| )
    // 파이프 한 쌍(| |)안에는 파라미터를 기술함 ( 하나 이상일시, | param1, param2 | 이렇게 표현)
    // 클로저는 함수의 결과값을 !저장하지 않음! 단지, 함수를 한 곳에 저장해 !나중에 사용!하는 것임.
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };  // 함수와 달리, let문을 완성시키기위하여 ; 사용

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)  // 여기서 한 번 함수 실행
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)  // 두번째 실행 
            // -> if 블럭 안에 클로저 호출 리턴 값을 저장하는 로컬 변수를 만들 수는 없을까? ->클로저 트레잇
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
*/


// 클로저는 !보통 짧고 좁은 문맥 안에서만 관련!
/*
모두 다 유효하고 같은 의미의 표현식임!
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;  // 클로저 바디에 단 하나의 표현식을 가져서 중괄호 지움

*** 오류
let example_closure = |x| x;

let s = example_closure(String::from("hello"));  // 컴파일러가 |x|를 String으로 추론, 고정
let n = example_closure(5);                      // |x|에 String대신 int가 들어와서 오류
*/



// 제네릭 파라미터와 Fn 트레잇을 사용하여 클로저 저장하기
// -> 변수에 클로저리턴을 저장하고 그 변수를 사용하는 것은 많은 반복된 코드를 만들 수 있음

// *** !memoization, lazy evaluation(지연평가)! 방식 사용
// -> 클로저와 호출한 결과값을 갖고 있는 구조체를 만들고, 결과값을 필요로 할 때만 그 구조체는
// 클로저를 호출함, 그리고 캐시에 저장, 필요할 때 사용

// 다만, 구조체에 클로저를 갖고 있도록 하기 위해 클로저 타입을 기술할 때, 각 클로저 인스턴스는
// !자신의 유일한 익명 타입!을 가짐 -> 동일한 시그니처지만 타입이 다르게 간주됨
// -> !제네릭!, !트레잇 바운드! 사용


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)  // 여기서 self.value 메소드 호출 
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)  // some(v)에서 v 반환
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32   // Fn 트레잇: 모든 클로저들은 Fn, FnMut, FnOnce 중 하나를 구현해야함!
                              // 여기서 Fn(u32) -> u32는, 파라미터(num) 타입이 u32이고, u32타입을 반환(num)하므로 이렇게 명시!
{
    calculation: T,     // 제너릭 타입 T (Fn이 구현되어있으므로 클로저임)
    value: Option<u32>, // 클로저 실행 전까지 value = None, 클로저의 결과 요청시 결과는 Some(var) 
}

// Cacher 구현의 제약사항: 항상 같은 값을 얻는다는 가정을 하고, 오직 u32타입을 파라미터와 리턴값으로 받음

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);  // v2 = 1 이므로 불가(바꿀 수 있는 메소드가 아님)
}







//impl에 트레잇 바운드를 붙인건가????????
// -> impl 바로 뒤에 T를 정의해야, 타입 Struct<T> 메소드를  구현할 때 사용할 수 있음... 복습하자...
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,  // match를 활용해, Some(v)값이 있다면 v 반환
            None => {      // 값이 None이라면, self.calculation에 저장된 클로저 호출
                let v = (self.calculation)(arg);  // 어케 되는 걸까...
                self.value = Some(v);
                v
            },
        }
    }
}
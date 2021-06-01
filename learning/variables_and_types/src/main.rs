fn main() {
    
    /*
    mut과 shadowing의 차이
    */

    let mut x = 5; // mut 선언, 자료형은 여전히 불변!
    println!("x의 값 {}", x);
    x = 6;
    println!("x의 값 {}", x);
    
    /* error[E0308]: mismatched types (자료형이 변함)
    x = "hello!"; 
    println!("x의 값 {}", x);
    */

    let x = 5;
    println!("x의 값 {}", x);
    let x = 6;  // shadowing (자료형도 달라질 수 있음)
    println!("x의 값 {}", x);
    
    // let으로 shadowing 했기에 OK! 이건 이름만 같은 !새로운! 변수, 여전히 !불변(immutable)!
    let x = "hello";
    //x = "bye"; error!
    println!("x의 값 {}", x);


    /*
    상수 선언하기
    : 
    상수는 mut키워드 사용 불가,
    let 대신 const 키워드 사용, 타입 어노테이션 반드시!!

    가독성을 위해 대문자에 공백은 _ 로 구분 (자릿수 표현에도 쓸 수 있다!)
    */

    const MAX_POINTS: u32 = 100_100;
    println!("상수: {}", MAX_POINTS);


    
    //데이터 타입 - Scalar, Compound
    // Rust는 정적 타입 언어로, 여러타입을 사용할 수 있을 때는 타입 어노테이션을 통해 데이터 타입 명시 필요
    let guess: u32 = "42".parse().expect("숫자가 아닙니다!");  // : u32 지울 시, !error[E0282]: type annotations needed  (consider giving `guess` a type) 발생
    println!("guess: {}", guess); // 42
    
    /*
    Scarlar type = [integer, floating point numbers, Booleans, characters(문자)]

    하나의 값을 표현하는 데이터 타입
    */


    // integer 소수점이 없는 숫자

    //let 8num_bit: i8 = 1;
    // println!("8num_bit: {}", 8num_bit); invalid suffix `num_bit` 앞에 접미사가 유효하지않음 -> 맨 앞을 숫자로 불가

    let num_bit_8: i8 = 127; // -128~127 2의 보수
    println!("numbit_8: {}", num_bit_8);  // 127
    
    let num_bit_32 =2_147_483_647;  // i32가 정수형 기본 데이터형
    println!("num_bit_32: {}", num_bit_32);

    let float_bit_64 = 9_223_372_036_854_775_807.0; //f64가 부동소수점 기본 데이터형
    println!("num_bit_64: {}", float_bit_64);
    
    
    // booleans 참거짓
    let t = true;
    let f: bool = false;
    println!("true: {}, false: {}", t, f);


    // char 문자타입
    let c = 'z';
    let z = '😄';
    print!("z: {}, smile: {}", c, z);

    /*
    Compound type = [tuples, array + Vector(std)]

    하나의 타입으로 여러 개의 값을 그룹화한 타입
    */

    // tuples 서로 다른 타입의 여러 값을 하나의 컴파운드 타입으로 그룹화하기에 적합

    let tup: (i32, f64, u8) = (32, 6.4, 8);  // 여러 타입을 넣을 수 있다!
    let (x, y, z) = tup;  // 개별 값을 읽기 위해서 !패턴 매칭! 사용
    println!("y의 값: {}", y);
    println!("index one: {}", tup.1);  // .i 로 인덱스로 참조도 가능!


    // array 튜플과 달리 각 요소는 반드시 같은 타입이어야 함! 

    let array_a: [i8; 5] = [1, 2, 3, 4, 5];  // array_a [i8: 5]는 i8타입으로 5개를 의미 (길이는 불변)
    let array_b = [3; 5];  // [3, 3, 3, 3, 3]

    println!("array_a_index_2: {}, array_b_index_1: {}", array_a[2], array_b[1])

}

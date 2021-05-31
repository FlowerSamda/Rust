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

}

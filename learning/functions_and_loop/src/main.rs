fn main() {

    println!("안녕하세요!");

    another_function();  // 먼저 정의되지 않아도 프로그램 내에서 불러오기에, !어디에 선언했는지!가 중요하다!
    num_sender(5);
    let x = return_value_fn();
    println!("return_value_fn: {}", x);


    
    let number = 3;
    
    if number < 5 {
        println!("조건이 일치합니다!");
    } else {
        println!("조건이 일치하지 않습니다!");
        }
    


}

fn another_function() {
    println!("또 다른 함수에요!!");
}

// parameter는 꼭 타입 명시를 해줘야한다!
fn num_sender(x: i32) {
    println!("x의 값: {}", x);
}

fn statements_and_expression() {

    /* 구문(statements)과 표현식(expression의 구분!)
    
     구문: 어떤 동작을 실행하지만, 값을 리턴하지는 않는 명령  -> let x = (let y = 6) 둘 다 구문이기에, let x에 (let y = 6)을 대입할 수 없어서 오류 (let y = 6 이건 구문이라 리턴 X)
     표현식: 최종 결괏값으로 평가(evaluate)됨, !표현식은 세미콜론을 사용하지 않음! 세미콜론 사용시 구문으로 바뀌어 값을 리턴하지 않는다!
    */

    let x = 6; // 구문(동작 실행, 값 리턴 X)
    let y = {  // 코드블록으로 감싸기 코드블록도 표현식임
        let x = 3;  // 구문임
        x + 1  // 표현식이므로 세미콜론 X
    };  // 하지만, 구문으로 변하기 위해 ; 사용!

}

fn return_value_fn() -> i32 {
    5
}

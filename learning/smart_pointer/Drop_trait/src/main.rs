// 스마트 포인터 패턴에서 중요한 두 번쨰 트레잇: Drop
// 값이 스코프 밖으로 벗어나려고 할 때 어떤 일이 발생될지를 커스터마이징

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // main을 벗어나면서, d -> c 순서대로 drop이 이뤄짐 (만들어진 순서의 역순으로 버려지기에)

    let e = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    // e.drop();   explicit destructor calls not allowed: 소멸자 명시 콜은 허용되지 않는다!
    // main 끝에서 drop호출을 여전히 자동적으로 할 것이기에, 2번 해제하면 중복 해제 에러 가능하기떄문
    // -> std::mem::drop 함수를 사용하여, 인자로 일찍 버리도록 강제할 값을 넘기면됨!
    drop(e);

    println!("CustomSmartPointer dropped before the end of main.");

}
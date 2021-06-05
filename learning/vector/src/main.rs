fn main() {
    // 벡터 Vec<T> : 메모리상에 서로 이웃하도록, 단일 데이터 구조 안에 여러 값 저장 (Heap 메모리)
    // struct와 마찬가지로, 벡터는 스코프 밖으로 벗어났을 때 드랍(내용물도!)

    // i32값의 비어있는 벡터 생성
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new(); // 데이터로부터 타입을 추론

    v.push(5); // v에 5를 넣었기에, i32로 러스트는 데이터 타입 추론
    v.push(6);
    v.push(7);
    v.push(8); // v = [5, 6, 7, 8]

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; //인덱스 문법으로 2번 인덱스 얻기
    println!("third: {}", third);

    let third: Option<&i32> = v.get(2); // get함수로 2번 인덱스 값 얻기
    println!("third: {:?}", third);

    // 벡터 범위를 벗어난 요소에 접근할 때
    //let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100];      // panic! (프로그램이 죽는 치명적 버그)
    //let does_not_exist = v.get(100);   // None 반환 (Option을 통해 다뤄야함)

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // 불변 참조자
    v.push(6); // 오류 (불변참조가 있는데 변경하려함)
               // 벡터의 동작 방식: 새로운 요소를 벡터 끝에 추가(push)하는 것은
               // 새로 메모리를 할당하여 예전 요소를 새 공간에 복사하는 일을 필요로 할 수 있음 (메모리에 저장할 공간이 충분치 않을 시)
               // 따라서, first 변수는 첫번째 인덱스라 상관이 없어보여도, 할당이 해제된 메모리를 가리키게 될 수 있으므로 오류
    // 벡터 내의 값들에 대한 반복 처리
    // for i in v 이렇게 함 (=파이썬)
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i)
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * : 가변 참조자(v)가 참조하고 있는 값을 바꾸기 위해, 역참조 연산자(*) 사용하여 값 얻어야함
    }

    // 열거형을 사용하여 여러 타입을 저장하기
    // 벡터는 같은 타입을 가진 값들만 저장할 수 있다고 했지만, enums의 variant는 같은 열거형 타입 내에 정의!
    // -> enums를 사용하면 다른 타입도 저장할 수 있다!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //추가로, pop은 마지막 인덱스를 반환하고 지워줌
    let last_index_return_and_remove = v.pop();
}

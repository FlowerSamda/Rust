fn main() {
    // 벡터 Vec<T> : 메모리상에 서로 이웃하도록, 단일 데이터 구조 안에 여러 값 저장 (Heap 메모리)
    // struct와 마찬가지로, 벡터는 스코프 밖으로 벗어났을 때 해제

    // i32값의 비어있는 벡터 생성
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new(); // 데이터로부터 타입을 추론

    v.push(5); // v에 5를 넣었기에, i32로 러스트는 데이터 타입 추론
    v.push(6);
    v.push(7);
    v.push(8);
}

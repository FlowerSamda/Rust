use std::collections::HashMap;

fn main() {
    
    // hash map 
    // HashMap<K, V> 타입은 K타입의 키에 V타입의 값을 매핑한 것을 저장
    // 벡터처럼, 인덱스가 아닌 임의의 타입으로 된 키를 이용하여 데이터를 찾을 때 유용 (=딕셔너리)
    // HashMap은 데이터를 힙에 저장,  벡터와 비슷하게 해쉬맵도 동질적임 (모든 키, 값은 같은 값!)

    let mut scores = HashMap::new();          //new를 이용하여 선언

    scores.insert(String::from("Blue"), 10);  //insert(key, value)
    scores.insert(String::from("Yellow"), 50);//이로써 scores는 key:String, value:int32

    // HashMap은 collect메소드를 사용하여 만들 수도 있음

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 각 이터레이션들을 zip으로 튜플로 묶음 -> vec![(Blue, 10), (Yellow, 50)]
    // 타입 명시 HashMap<_, _>는 collect가 다른 많은 데이터 구조로 바뀔 수 있고, 러스트는 우리가 타입을 특정하지 않으면 어떤 것을 원하는지 모름.
    // 키와 값의 타입에 대한 타입 파라미터에 대해서는 밑줄을 쓸 수 있으며 러스트는 벡터에 담긴 데이터의 타입에 기초하여 해쉬에 담길 타입 추론
    // HashMap<&String, &i32>를 해야하는 것을 보면, zip은 참조 값을 던지나 봄
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for c in scores {
        println!("{:?}", c)
    }

    // 해쉬맵과 소유권
    // 키와 값이 삽입되는 순간, 이들이 해쉬맵의 소유가 된다!
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);  // 해쉬맵으로 이동시킨 후, 더이상 이 둘(key, value)을 사용할 수 없음
    // println!("{}", field_name)  오류 : map.insert에서 소유권이 넘어감!

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}, {}", key, value)
    }

    // 값 덮어쓰기  (이미 존재하는 키값쌍에 새로운 값 삽입시)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);   // 25출력

    // 키에 할당된 값이 없을 경우에만 삽입하기
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    //entry의 리턴 값은 Entry이고, 타입은 enums.
    // 만약 키가 있다면 그 값을 반환하고, 키가 없다면, or_insert()메서드로 키에 대한 값을 넣음
    scores.entry(String::from("Yellow")).or_insert(50);  // 새로운 키&값
    scores.entry(String::from("Blue")).or_insert(50);    // 이미 존재하는 키&값

    println!("{:?}", scores);  // {"Blue":10, "Yellow":50}


    // 예전 값을 기초로 값을 갱신하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();  // HashMap 객체 선언

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // word가 있다면 그냥 반환, 없으면 word:0 키값쌍을 넣음
        *count += 1;  // *(애스터리스크)를 사용한 역참조로 +=1 함 약간 파이썬 딕셔너리 get 사용하는 느낌 근데 이거 가변인가?
    }

    println!("{:?}", map);
}   

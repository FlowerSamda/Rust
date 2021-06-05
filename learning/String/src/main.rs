fn main() {
    // 비어있는 String 만들기
    let mut s = String::new();

    // 초깃값을 가지고 String 만들기  !3가지 모두 가능하다!
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // String은 UTF-8로 인코딩되어있기에, 인코딩된거라면 무엇이든지 가능하다!
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //스트링 슬라이스를 String에 추가하기
    let mut s = String::from("foo");
    s.push_str("bar");  // push_str은 스트링슬라이스(str)를 파라미터로 가짐(파라미터의 소유권이 필요 없어서)

    //push를 사용하여 String값에 한 글자 추가하기
    let mut s = String::from("lo");
    s.push('l'); // 작은 따옴표(')를 사용해야한다, s = "lol"

    //연산자나 format! 매크로를 이용한 접합
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // 연산자 + 사용
    let s3 = s1 + &s2;   // s1은 여기서 소유권이 s3에게로 넘어감, s2는 사용 가능
    println!("{}", s2);  //가능, s1은 불가
    println!("{}", s3);  // Hello, world!

    /*
     + 연산자로 String + &String을 했지만, 메서드인 add가 호출된다.
    fn add(self, s: &str) -> String {~~
    이것처럼, 인자 s (여기선 &s2)는 &str인데, s2가 &String인데도 컴파일이 되는
    이유는 이후에 배울 !역참조 강제! 때문. add는 결국 참조변수를 가져가기에, s2는 연산 이후에도 유효
    다만, 첫 인자로 self를 가져가기에, s1은 메서드로  소우권이 넘어가고 메서드가 끝나면 drop된다.
    즉, 복사본을 만드는 것처럼 보여도 그렇지 않기 때문에 !복사보다 더 효율적!

    */

    // 연산자 +를 여러 String과 함께 쓸 때
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // (((s1 + 강제&str) + &s2) + 강제&str) + &str 이런 식인듯? => 개불편
    let s = s1 + "-" + &s2 + "-" + &s3;
    

    // 여러 String을 접할할 때는 format! 매크로 사용하기
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);  // s: String, 소유권을 안가져감


    // 스트링 내부 인덱싱 : String객체에 인덱싱 문법 사용시 에러를 얻음
    let s1 = String::from("hello");
    //let h = s1[0]; 'String' cannot be indexed by `{integer}`
    // 이유:
    // ** String의 내부적 표현 **
    // String은 Vec<u8>을 감싼 것 (Wrapper)

    // 이 경우, len = 4인데, 이는 String "Hola"를 저장한 Vec이 4바이트 길이라는 뜻
    // (UTF-8로 인코딩시 각각 글자가 1 바이트라는 뜻)
    let len = String::from("Hola").len();

    // 이 경우, len은 24임 ( 각각 유니코드 스카랄 값이 저장소의 2바이트를 차지하므로)
    // => 스트링의 바이트들 안의 인덱스는 유효한 유니코드 스칼라 값과 대응되진 않음
    let len = String::from("Здравствуйте").len();

    // 유효하지 않은 러스트 코드 예제
    // UTF-8로 인코딩시, З의 첫번째 바이트는 208, 두번째는 151이므로, 208로 예상할 수 있지만, 208은 그 자체로 유효하진 않음
    // 그래서, &"hello"는 h가 아니라 104(바이트 값)를 반환 => 직관적이지 않기에 버그 발생 가능 (문자열을 예상했는데 바이트가 나온다!?)
    //let hello = "Здравствуйте";
    //let answer = &hello[0];

    
    // String 슬라이싱 하기
    // 종전의 문제 때문에, 러스트는 스트링 슬라이스를 위해 정말로 인덱스를 사용한다면, 조금 더 구체적으로 지정하길 원함

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // 이런 식으로, [0]처럼 숫자 하나를 사용하는 게 아니라, []와 범위를 사용하여 특정 바이트들을 담고 있는 스트링 슬라이스를 만들 수 있음
    //만약, &hello[0..1] 이런 식으로, 유효하지 않은 바이트를 원하면, 벡터에서 유효하지 않은 인덱스에 접근한 것처럼 패닉 발생

    
    // String 내에서 반복적으로 실행되는 메소드
    // 스트링 내 개별적인 유니코드 스칼라 값에 대한 연산을 수행하길 원한다면, 가장 좋은 방법은 chars 메소드를 사용하는 것
    // chars 메소드를 유니코드 스칼라값에 호출하면, char타입으로 나누어서 반환, 이 결과값에 대해 반복(iterate)해서 접근 가능
    for c in "hello".chars() {
        println!("{}", c);
    }
    // bytes 메소드를 사용시, 가공되지 않은 각각의 바이트 반환
    // 다만, 유효한 바이트 값인지는 모름 (단일 스칼라 값이 2개 이상 바이트로 구성 시)
    for b in "hello".bytes() {
        println!("{}", b);
    }

    
}


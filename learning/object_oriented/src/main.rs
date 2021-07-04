// 객체 지향 디자인 패턴 구현하기

// 상태 패턴(state pattern)은 객체 지향 디자인 패턴임.
// -> 어떤 값이 "상태 객체"들의 집합으로 표현되는 일종의 내부 상태를 가지며,
//    이 값은 내부 상태에 기반하여 바뀐다는 것

// 즉, 각 상태 객체는 자신의 동작 및 다른 상태로 변경되어야 할 때의 제어에 책임이 있음
// 값이 다뤄지는 각 과정에서 한 상태 객체를 보유한 값은 상태들의 서로 다른 행동 혹은 
// 상태 간의 전이가 이뤄지는 때에 대해 아무것도 모름
// -> 한 상태 객체


// 블로그에 게시물을 올리는 작업 흐름을 구현하기

// 순서
// 1. 블로그 게시물은 빈 초안으로 시작
// 2. 초안이 완료되면 게시물의 검토가 요청됨
// 3. 게시물이 승인되면 게시됨
// 4. 오직 게시된 블로그 게시물만이 출력될 내용물을 반환하기에, 승인되지 않은 게시물은 무조건 게시 불가

extern crate object_oriented;
use object_oriented::blog::Post;

fn main() {
    /* 타입인코딩이 없을시 코드

    // 오직 Post 타입과만 상호작용하는데, 이 타입은 상태 패턴을 사용하며 게시물이 가질 수 있는
    // 초안, 리뷰 대기중, 게시됨을 나타내는 세가지 상태 중 하나가 될 값을 보유

    let mut post = Post::new();       // 초안을 Post::new 를 통해 만들도록 허용하기

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());   // 검토전 콘텐츠는 빈 문자열

    post.request_review();
    assert_eq!("", post.content());   // 검토가 되기 전까지도 빈 문자열

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());  // 승인 후 표시가 됨
    */

    let mut post = Post::new();

    post.add_text("I'm Rustacean");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I'm Rustacean", post.content())

}

// 결론

// 객체 지향 프로그래밍에서 나오는 패턴을 러스트에서도 구현할 수 있음

// 근데, 러스트는 탄탄한 컴파일러로 모든 에러를 잡아주는데, 굳이 상태 흐름같은 방법을 안쓰고도
// 소유권같은 러스트만의 장점으로 코딩할 수 있기에, 코딩 전에 한번 생각해보기!
pub struct Post {
    content: String,
}

// 상태와 동작을 타입처럼 인코딩하기
// -> 상태는 결국 타입(구조체)에 의해 구별된다!
// content 메소드를 가지지 않게하여, 메소드 접근시 존재하지 않는다는 컴파일 에러 반환
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // self를 취하므로, DraftPost를 소비하여 PendingReviewPost로 변환
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // self를 취하므로, PendingReviewPost를 소비하여 Post로 변환
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// 상태 패턴의 장점
// Post의 request_review 메소드는 이것의 state가 무엇이든 상관없이 동일하게 반환

impl Post {

    /*
    // 기본적으로 Post 객체는 비공개이기에, 다른 상태로 Post를 생성할 방법은 없음!
    pub fn new() -> Post {
        Post {
            // Post 객체에 state, content 설정
            state: Some(Box::new( Draft {} )),  // Draft 구조체의 새 인스턴스를 가진 Box를 보유한 Some 값
            content: String::new()
        }
    }
    */

    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    // &str을 넘김으로써 블로그 게시물의 콘텐츠 글에 추가할 수 있도록 하는 메소드
    // &mut self를 취하는 이유는 add_text를 호출하는 Post 인스턴스를 변경하게 되기 때문
    // 이 메소드는 state 필드와 전혀 상호작용을 하지 않지만, 우리가 지원하고자 하는 동작 요소
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 게시물이 검토 완료 상태 전까지는 빈 스트링을 반환해야하기에 항상 비어있어야함!
    pub fn content(&self) -> &str {
        // state.as_ref()는 무조건 Some(&T)이기에, unwrap()으로 패닉이 절대 발생하지 않음
        // self.state.as_ref().unwrap().content(&self)

        // 안정적이게 변경 (DraftPost 구조체 사용)
        &self.content
    }

    /* 이전에 상태라는 필드를 사용한 코드


    // Post의 현재 상태 상에서 이 메소드를 호출하면, 현재의 상태를 소비하고 새로운 상태를 반환
    pub fn request_review(&mut self) {
        //take 메소드를 호출하여 state 필드 밖으로 Some을 빼내고 None을 남김 (러스트는 구조체에 값이 없는 필드를 허용하지 않음)
        //Post가 예전 state값을 사용할 수 없게 하기 위하여, state값을 빌리기보다는 게시물 밖으로 이동시키도록 만듦.
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())  // Draft.req~~ or PendingReview.req~~~ ...
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())         // Draft.app~~ or PendingReview.req~~~ ...
        }
    }
    */

}

// State 트레잇은 게시물의 상태 변화에 따라 달라지는 동작(Draft, PendingReview, Published)을 정의
trait State {
    // self: Box<self>를 취하는 이유는 메소드가 오직 그 타입(self)을 보유한 Box상에서 호출할 경우에만 유효!
    fn request_review(self: Box<Self>) -> Box<State>;   // request_review라는 함수를 꼭 가져야 함
    // self:Box<self> 문법은 Box<Self>의 소유권을 가져가기에, Post의 예전 상태를 무효화해 새 상태로 변화하게 해줌.
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""  // 역참조 강제를 통해 Box<State>의 content 함수를 구현한 것 (기본 구현)
    }
}

struct Draft {}   // 초안

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})  // PendingReview 객체를 통해, 검토를 기다리는 상태로 만듦
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self  // 아무 효과 없이 자기 자신이 반환됨.
    }
}

struct PendingReview {}  // 검토 대기

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self  // 리뷰를 요청할 때는 이 상태를 그대로 유지해야함 -> 아무런 변경도 하지 않음
    }
    fn approve(self:Box<Self>) -> Box<State> {
        Box::new(Published {})    // 검토의 다음 단계는 게시 이므로!
    }
}


struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<State> {
        self
    }

    // content는 Post의 라이프타임에 영향을 받기에 post와 함께 라이프타임 명시
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content  // State의 content 메소드를 오버라이딩
    }
}


// "상태 패턴"에 따라 3개의 struct(Draft, PendingReview, Published)에 트레잇의 동일한 구현을 통하여
// 항상 같은 결과가 나오도록 함으로써 안정적인 프로그래밍이 가능하다!!



// 상태 패턴의 기회비용

// 각 상태에 가져야 하는 서로 다른 종류의 동작을 캡슐화하기 위하여 객체지향 상태 패턴을 구현함
// 게시된 게시물이 취할 수 있는 서로 다른 방법을 알기 위해선,
// 단 한 곳(Published 구조체의 State 트레잇)만을 확인하면 됨!

// 다만, 각 상태를 다루는 구조체에 중복된 코드가 들어가는 것을 State 구조체에서 self로 넣어버리면,
// 어느 구체적인 self인지 알 수 없으므로 객체 안전성이 위배될 수 있어서, 어쩔 수 없이 중복이 생김.


// 객체 지향언어에서 정의하는 것과 동일하게 상태 패턴을 구현함으로써, 우리가 사용할 수 있는 러스트의
// 강점을 모두 이용하지 못하고있기에, 유효하지 않은 상태 및 전환이 컴파일 타임 에러가 될 수 있도록
// 하기 위하여, blog crate에 변경 시작
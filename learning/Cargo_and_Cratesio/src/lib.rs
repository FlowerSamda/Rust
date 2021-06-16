// 유용한 문서화 주석 만들기
// https://rinthel.github.io/rust-lang-book-ko/ch14-02-publishing-to-crates-io.html 참조~!

//! 이렇게 시작하면, 주석을 포함하는 항목을 문서화하게 됨! 이 경우, 주석을 포함하는 항목은 루트파일이고,
//! 주석은 전체 크레이트를 설명하게 된다! (문서 전체의 주석이라고 생각하면 됨)

/// 크레이트가 어떻게 구현되었는지가 아닌, !어떻게 사용하는지!를 알려줌
/// Adds one to the number given.
///
///
/// 
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}



//만약 이 상태로 하면, 페이지에는 kinds와 utils만 보이고, kinds 내의 열거자는 보이지 않아서
//사용하게 되는 사람이 불편함!!

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --생략--
    }
}

// Re-export: 기존 private 구조와 다른 public 구조를 만들 수 있다는 듯!
// 즉, 한 위치에서 공개 항목을 가져오고, 이것을 마치 다른 위치에서 정의한 것처럼 공개항목으로 만드는 것!

extern crate art;

//use art::kinds::PrimaryColor;  // 사용하는 사람은 art내에 kinds와 utils같은 게 있음을 알 필요가 없음...
//use art::utils::mix;           // 왜냐하면 PrimaryColor와 mix만 사용하기 때문

pub use kinds::PrimaryColor;     // 따라서, pub use문으로 re-export하도록 art 크레이트 코드를 수정해야함!
pub use kinds::SecondaryColor;   // 이렇게하면, API문서에서 Re-exports목록과 링크가 첫 페이지가 나타나서
pub use utils::mix;              // 각 연관함수를 쉽게 볼 수 있다!

// 결론적으로, 만약 특정 부분에서 중첩된 모듈이 많은 경우, 모듈의 상위 계층에서 pub use를 이용해
// 타입을 다시 export함으로써, 사용자들에게 더 뛰어난 경험을 제공할 수 있음.


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
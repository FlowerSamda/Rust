pub mod client;
pub mod network;
//  중첩구조(nested)의 모듈 만들기
/*
mod network {
    fn connect() {

    }

    mod server {
        fn connect() {

        }
    }
}
*/

// 비공개 규칙
// 1. 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근 가능
// 2. 어떤 아이템이 비공개라면, 이는 같은 파일 내 부모모듈 및 부모 모듈의 자식 모듈에서만 접근 가능

// outermost는 프로젝트 루트 모듈 내에 있음
/*
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}
*/
// 1. outermost 모듈자체는 현재 루트 내에 있는 try_me에서 접근 가능
// 2. middle_function은 공개이므로, outermost 모듈의 바깥에 있는 try_me에서 접근 가능
// 3. middle_secret_function은 비공개이므로, outermost 모듈의 바깥에 있는 try_me에서 호출 불가능
// 4. inside 모듈은 비공개이므로, 애초에 try_me가 outermost의 바깥에 있기에 접근 불가
/*
fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();  // private function error
    outermost::inside::inner_function();  // private module error
    outermost::inside::secret_function(); // private module error
}

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}  -> private function
    // inside 모듈 pub 설정
    pub mod inside {
        pub fn inner_function() {}

        fn secret_function() {}     -> private function
    }
}
*/
// outermost를 pub으로 설정 -> 짜피 같은 모듈이라 접근할 수 있었어서, 차이 없음
/*
pub mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}  -> private function
    // inside를 private 그대로
    mod inside {
        pub fn inner_function() {}  -> private module

        fn secret_function() {}     -> private module
    }
}
*/

// inner_function 내부에서 middle_secret_function 호출
/*
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}  -> private function
    // pub 설정
    pub mod inside {
        pub fn inner_function() {
            // 이 함수 내부에서 secret_function 호출
            secret_function()       -> 간웅
        }

        fn secret_function() {}     -> private function
    }
}
*/

// 여기서부터 보기 https://rinthel.github.io/rust-lang-book-ko/ch07-02-controlling-visibility-with-pub.html
/*
pub mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            //outermost::middle_secret_function()
            //앞의 :: 는 이 루트 모듈로부터 시작하여 모듈 참조를 하겠다는 것
            //이거 외않되
        }

        pub fn secret_function() {}
    }
}
*/

// 각 모듈들은 "현재" 모듈을 기준으로 상대적!
// ex) test 모듈 내에서는 communicator가 위에있다!
//communicator
// ├── client
// ├── network
// |   └── client
// └── tests




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::client;
        // super : 계층구조 상 현재 모듈로부터 한 모듈 올라감
        // use super::something시, 부모모듈에 상대적인 경로가 되게 해줌
        client::connect();
    }
}

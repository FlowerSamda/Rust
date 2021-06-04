// 이름이 mod인 이유는 디렉토리의 이름을 따라오는 듯?? 
// => 그냥 원래 그런 듯 https://rinthel.github.io/rust-lang-book-ko/ch07-01-mod-and-the-filesystem.html
pub mod server;

pub fn connect() {
}

// server는 network의 서브모듈이므로 mod 선언 필요
/*
mod server {
    fn connect() {
    }
}
*/
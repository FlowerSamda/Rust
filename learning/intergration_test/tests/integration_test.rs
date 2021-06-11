// extern crate를 사용한 이유
// -> tests디렉토리 내의 각 테스트가 모두 개별적인 크레이트이기 때문!
extern crate intergration_test;
// 셋업 파일을 모듈로 사용할 수 있게 하기
// tests 파일 내에 모듈 사용처럼 폴더를 만든 후, 그 곳에 mod.rs 생성
mod common;

// #[cfg(test)]를 선언하않은 이유는, 카고는 tests디렉토리를 특별취급하여, cargo test를 실행했을 때만 이 디렉토리 내 파일을 컴파일해서임.

#[test]
fn it_adds_two() {
	common::setup();
    assert_eq!(4, intergration_test::add_two(2));
}


// 결과 값 

/*
root@goorm:/workspace/Rust/Rust/learning/intergration_test(master)# cargo test
   Compiling intergration_test v0.1.0 (/workspace/Rust/Rust/learning/intergrati
on_test)
    Finished test [unoptimized + debuginfo] target(s) in 6.40s
	*** 유닛테스트!
     Running unittests (target/debug/deps/intergration_test-e1493e4219920041)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fin
ished in 0.00s
*** 통합테스트
     Running tests/integration_test.rs (target/debug/deps/integration_test-cee6
b901e7cea727)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fin
ished in 0.00s
*** 문서 테스트
   Doc-tests intergration_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; fin
ished in 0.00s
*/


// 바이너리 크레이트를 위한 통합 테스트:
// -> src/lib.rs가 없고 src/main.rs만 있다면, src/lib.rs의 파일을 가져와서 main.rs에서 호출하는 그런 식으로 테스트 가능 (lib을 통합테스트한다면, main은 소량의 코드이므로 테스트 필요 X)


// 더 많은 내용
// https://rinthel.github.io/rust-lang-book-ko/ch11-03-test-organization.html 





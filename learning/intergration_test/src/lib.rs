pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
	use super::*;
	
	// internal_adder함수는 pub으로 표시되어있지 않지만, tests모듈도 그냥 또 다른 모듈이고, 그냥 러스트 코드라서, 그냥 됨.(대충 테스트라는 뜻)
    #[test]
    fn it_works() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}

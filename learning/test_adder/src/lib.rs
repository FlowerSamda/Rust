#[cfg(test)]
mod tests {
	
    #[test]  // 이 속성은 아래 함수가 테스트 함수라는 어노테이션(속성), 그냥 일반적인 셋업함수나 일반적 함수가 아니라 테스트를 한다는 것을 나타냄.
    fn exploration() {
        assert_eq!(2 + 2, 4);  // 어떤 조건이 true임을 보장하기를 원할 경우 유용! `
    }
	
	/*
	#[test]  // panic으로 인해 실패할 test
	fn another() {
		panic!("Make this test fail!!!")
	}
	*/
	
	//assert! 매크로 사용하기
	#[derive(Debug)]
	pub struct Rectangle {
		length: u32,
		width: u32,
	}

	impl Rectangle {
		pub fn can_hold(&self, other: &Rectangle) -> bool {
			self.length > other.length && self.width > other.width
		}
	}
	
	
	#[cfg(test)]
	mod tests {
		use super::*;
		
		#[test]
		fn larger_can_hold_smaller() {
			let larger = Rectangle {length: 8, width: 7};
			let smaller = Rectangle {length:5, width:1};
			
			assert!(larger.can_hold(&smaller));
		}
		
		#[test]
		fn smaller_cannot_hold_larger() {
			let larger = Rectangle {length: 8, width: 7};
			let smaller = Rectangle {length:5, width:1};
			
			assert!(!smaller.can_hold(&larger));  // !false -> true
		}
	}
	
	
	// assert_eq!, assert_ne! 사용하기
	// assert_eq!, assert_ne!는 각각 ==와 !=를 이용하고, 단언에 실패하면 디버그 포맷팅을 사용하여 출력
	// -> Debug와 PartialEq 트레잇을 구현할 필요가 있음
	// 이는 [#derive(PartialEq, Debug)] 어노테이션을 정의에 추가함으로써 가능
	pub fn add_two(a: i32) -> i32 {
		a + 2
	}
	
	#[cfg(test)]
	mod tests2 {
		use super::*;
		
		#[test]
		fn it_adds_two() {
			assert_eq!(4, add_two(2));
		}
		
		#[test]
		fn it_adds_three() {
			assert_ne!(5, add_two(2))
		}
		
		
	// 커스텀 실패 메세지 추가하기
		pub fn greeting(name: &str) -> String {
			format!("Hello {}!", name)
			// String::from("Hello!") 오류를 일으키는 코드
		}

		#[cfg(test)]
		mod tests {
			use super::*;

			#[test]
			fn greeting_contains_name() {
				let result: String = greeting("Carol");
				//assert!, assert_eq(ne)! 매크로는 하나(assert) 또는 두개(eq, ne)의 파라미터 다음에 받는 파라미터들은 format! 매크로에 넘겨짐 -> {}를 갖는 포맷 스트링과 이 변경자에 입력될 값들을 넘길 수 있음.
				assert!(
					result.contains("Carol"),
					"Greeting did not contain name, value was '{}'", result  // 필수 파라미터 뒤의 format! 파라미터들
				);
			}
		}
	}
		
		
	
	// should_panic을 이용한 패닉에 대한 체크
	// 패닉이 나야 할 곳에 제대로 나는지 확인하는 어노테이션
		
		pub struct Guess {
			value: u32,
		}
		
		impl Guess {
			/* 엄밀하지 않은 #[should_panic] 위한 코드
			// value가 1보다 작거나 100보다 크면 패닉을 일으킴
			pub fn new(value: u32) -> Guess {
				if value < 1 || value > 100 {
					panic!("Guess value must be between 1 and 100, got {}.", value);
				}
				
				// Guess에 value:value를 가진 필드 생성
				Guess {
					value
				}
			}
			*/ 
			
			// 엄밀한 should_panic(expected~~ 사용)
			pub fn new(value: u32) -> Guess {
				if value < 1 {
            		panic!("Guess value must be greater than or equal to 1, got {}.",
                   	value);
				} else if value > 100 {
				// panic!의 메세지로 전달되는 스트링의 서브스트링을 사용함
				// (should_panic(expected=~~))에서 ~~가 에러 메세지의 서브스트링이면 통과!
            		panic!("Guess value must be less than or equal to 100, got {}.",
                   	value);
        		}
				
				Guess {
					value
				}
			}
		}
		
		#[cfg(test)]
		mod tests3{
			use super::*;
			
			#[test]
			// 코드가 패닉을 일으키면 pass, 아니면 fail  하지만, 우리가 예측치 못한 곳에서 패닉이 나도 pass이기에,
			//#[should_panic]
			// -> 더 엄밀하게 사용하기 위해서는 should_panic 속성에 expected 파라미터를 추가할 수 있음.
			// expected의 값이 에러메세지의 서브스트링이면 통과!
			#[should_panic(expected = "Guess value must be less than or equal to 100")]
			// 패닉 메시지가 expected의 스트링을 포함하지 않는다면, Panic did not expected string ~~~. 메시지 반환
			fn greater_than_100() {
				Guess::new(200);
			}
		}
		
		
		
		
		
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
	
}

fn main() {
	
	
	// Lifetime의 주 목적은 댕글링 참조자(dangling reference)를 방지하는 것
	// 댕글링 참조자는 프로그램이 우리가 참조하기로 의도한 데이터가 아닌 다른 데이터를 참조하는 원인이 됨.
	
	/* 이미 내부스코프가 끝났는데, 내부스코프에서 정의한 r을 출력하려고 시도하는 예시
	
	{
		let r;

		{
			let x = 5;
			r = &x;       // borrowed value does not live long enough 오류!   
		}                 // drop

		println!("r: {}", r);
	}
	*/
	
	
	// 함수의 제네릭 라이프타임
	
	let string1 = String::from("abcd");
	let string2 = "xyz";
	
	let result = longest(string1.as_str(), string2);
	println!("The longest string is {}", result);
	
	// 소유권을 얻는 걸 원치 않기에, 스트링 슬라이스(스트링 슬라이스 = 참조자)를 파라미터로 받음
	// 오류 발생 이유: 반환되는 참조자가 x를 참조하는지, y를 참조하는지를 러스트가 말할 수 없음.
	// 함수명<'a>, 각 파라미터와 반환 값에 'a 라이프타임 지정시 오류 해결!
	fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
		                                        // ^ 라이프타임 미정의시 expected named lifetime parameter 오류 발생
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}
	
	// 라이프타임 명시는 함수 시그니처가 제네릭 타입 파라미터를 특정할 때 이 함수가 어떤 타입이든 허용할 수 있는 것과 같은 방식.
	// -> 함수 시그니처가 라이프타임 파라미터를 특정할 때라면, 이 함수는 어떠한 라이프타임을 가진 참조자라도 허용할 수 있음
	/*이 함수 시그니처는 이제 어떤 라이프타임 'a에 대하여, 이 함수는 두 개의 파라미터를 갖게 될 것인데, 두 개 모두 적어도 라이프타임 'a만큼 살아있는 스트링 슬라이스임을 말해줍니다.
	이 함수는 또한 적어도 라이프타임 'a만큼 살아있는 스트링 슬라이스를 반환할 것입니다. 이는 러스트에게 우리가 강제하고 싶은 것을 말해주는 계약입니다.*/
	// 함수 시그니처에 붙이기만 하면 되는 이유는, 함수 내는 컴파일러가 검사할 수 있고, 함수가 그 함수 밖 코드의 참조자를 가지고 있으면, 인자들 혹은 반환값들의 라이프타임이 함수가 호출될 때마다 달라질 가능성이 있어서임.
	
	
	/*
	&i32         // a reference.
	&'a i32      // a reference with an explicit lifetime.
	&'a mut i32  // a mutable reference with an explicit lifetime.
	*/
	
	// 라이프타임 'a를 가지고 있는 i32에 대한 참조자인 first와, 'a를 동일하게 가지고 있는 i32에 대한 참조자 second가 주어질 시
	// -> 참조자 first와 second는 동일한 제네릭 라이프타임만큼 살아야 한다는 것을 의미함!
	
	
	// longest함수에 라이프타임을 명시했기떄문에, 외부 스코프의 끝까지 string2가 유효할 필요가 있다는 오류 발생 예시
	let string1 = String::from("long string is long");            // string1 정의
    let result;
    {
        let string2 = String::from("xyz");                        // string2 정의
        result = longest(string1.as_str(), string2.as_str());     // 오류 발생 지점(not live long enough)
    }                                                             // string2의 드랍 지점
    //println!("The longest string is {}", result);               // borrow later used here
	
	
	// 라이프타임의 측면에서 생각하기!
	// 라이프타임 파라미터를 특정하는 정확한 방법은, 함수가 어떤 일을 하고 있는가에 따라 달린 문제!
	// ex) longest함수의 구현을 제일 긴 스트링 슬라이스 대신 항상 첫번째 인자를 반환하도록 했을 시 -> 라이프타임 노필요
	
	// y의 라이프타임을 특정하지 않은 이유는, y의 라이프타임은 x의 라이프타임과 어떠한 연관도 없음!
	fn longest_2<'a> (x: &'a str, y: &str) -> &'a str {
		// 반환값(&'a str)에 라이프타임을 지정한 이유는, 반환되는 참조가 인자들을 중 하나를 참조하지 않는다면...?
		// -> 유일한 가능성은 함수 내에서 생성된 값을 참조 -> 함수가 끝나는 시점에 스코프 밖으로 가서 댕글링 참조자가 됨...
		// -> 컴파일 거부
		x
	}
	
	
	// 반환값이 인자의 라이프타임을 참조하지 않을 때 (&'a str로 선언했더라도, 인자와 아무 관련이 없음 이유는 아래)
	fn longest_3<'a>(x: &str, y: &str) -> &'a str {
    	let result = String::from("really long string");
    	result.as_str()                                   // result는 여기서 참조, 반환 -> 인자(<'a>)와 관련 X
	}                                                     // 여기서 메모리 해제, 댕글링 참조자가 되어버림
	// -> 가장 안전한 방법은, 참조자보다는 차라리 값을 소유한 데이터 타입을  리턴하도록 하여, 호출하는 함수가 값을 할당해제하게함.
	
	
	
	
}




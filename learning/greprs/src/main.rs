extern crate greprs;

use std::env;
use std::process;
use greprs::Config;
use greprs::run;

fn main() {
    
	/*
	// 커맨드라인 인자 허용하기
	
	// env::args(): 반복자 형식으로 커맨드라인 인자들을 우리 프로그램에 전달
	let args: Vec<String> = env::args().collect();  // args = ["target/debug/greprs", "hello", "123"]
	// 벡터의 첫번째 값은 바이너리의 이름(target/debug/greprs)
	
	
	// unwrap_or_else : Ok시는 unwrap과 유사(값 반환), Err 시 closure의 코드 호출
	let config = Config::new(&args).unwrap_or_else(|err| {
		// Config::new에서 반환된 값이 Err이면, 클로저를 호출하고, 클로저는 익명의 함수로 unwrap_or_else메소드의 인수로 전달됨. 이후, err값을 |err|로 전달하고 출력. (13장)
		eprintln!("Problem parsing arguments: {}", err);  // unwrap: Ok ->반환, Err ->패닉, eprintln!으로 표준에러 사용
		process::exit(1);  // 프로그램을 즉시 중단시키고, 종료상태 코드로 전달받은 값(1)을 반환
		});
	*/

	// Iterator 적용 후 (13장)
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
		process::exit(1);
	});
	
	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);
	
	if let Err(e) = run(config) {
		// eprintln!을 사용하여 표준에러를 사용하고, "> 파일명.txt" 문법을 커맨드라인에 사용하면,
		// 표준출력이 파일에 저장됨( > 파일명.txt ) 문법은 표준출력이 파일로 가게 만드는 것.
		eprintln!("Application error: {}", e);
		
		process::exit(1);
	}
}

// cargo run ~~~ > 파일명.txt
// -> 표준출력의 내용을 화면이 아닌 파일명.txt에 출력하게끔 하는 것

// 에러만 터미널에 출력하고, 결과물을 파일에 저장할 때 유용!!!














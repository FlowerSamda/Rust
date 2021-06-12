extern crate greprs;

use std::env;
use std::process;
use greprs::Config;
use greprs::run;

fn main() {
    
	// 커맨드라인 인자 허용하기
	
	// env::args(): 반복자 형식으로 커맨드라인 인자들을 우리 프로그램에 전달
	let args: Vec<String> = env::args().collect();  // args = ["target/debug/greprs", "hello", "123"]
	// 벡터의 첫번째 값은 바이너리의 이름(target/debug/greprs)
	
	// unwrap_or_else : Ok시는 unwrap과 유사(값 반환), Err 시 closure의 코드 호출
	let config = Config::new(&args).unwrap_or_else(|err| {
		// Config::new에서 반환된 값이 Err이면, 클로저를 호출하고, 클로저는 익명의 함수로 unwrap_or_else메소드의 인수로 전달됨. 이후, err값을 |err|로 전달하고 출력. (13장)
		println!("Problem parsing arguments: {}", err);  // unwrap: Ok ->반환, Err ->패닉
		process::exit(1);  // 프로그램을 즉시 중단시키고, 종료상태 코드로 전달받은 값(1)을 반환
		});
	
	println!("Searching for {}", config.query);
	println!("In file {}", config.filename);
	
	if let Err(e) = run(config) {
		println!("Application error: {}", e);
		
		process::exit(1);
	}
}
















use std::env;
use std::fs::File;        // 파일 관련 라이브러리
use std::io::prelude::*;  // 파일 I/O를 포함한 I/O작업에 유용한 라이브러리
use std::process;

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
	
	// 파일 I/O 시작
	let mut f = File::open(filename).expect("file not found");
	
	let mut contents = String::new();  // 읽어들인 내용 보관 용도
	f.read_to_string(&mut contents).expect("Something went wrong reading the file");
	println!("with text:\n{}", contents);
}

struct Config {
	query: String,
	filename: String,
} 

impl Config {
	fn new(args: &[String]) -> Result<Config, &'static str> {
		
		if args.len() < 3 {
			// Err로 포장된 문자열 리터럴을 반환하는데, 문자열 리터럴은 &'static str 임! (10장)
			return Err("not enough arguments");
		}
		
		let query = args[1].clone();    // 파라미터 args는 main의 args의 참조값만 가지기에, 복사본으로 주기
		let filename = args[2].clone(); // 기회비용이 있지만, 작은 데이터라 그냥 가져가보기
		
		Ok(Config { query, filename })
	}
}


// run 함수 추출하기부터 하기

















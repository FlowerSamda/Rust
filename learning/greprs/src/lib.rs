// 기존 함수, 구조체들을 크레이트화!

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive_env: bool,
	pub case_sensitive_args: bool,
} 
// 개인과제: 환경변수(var), 커맨드라인 인수(args) 둘 다 받는데, 그 우선순위를 조절해보기
impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		args.next();

		/*
		if args.len() < 3 {
			// Err로 포장된 문자열 리터럴을 반환하는데, 문자열 리터럴은 &'static str 임! (10장)
			return Err("not enough arguments");
		}
		
		let query = args[1].clone();    // 파라미터 args는 main의 args의 참조값만 가지기에, 복사본으로 주기
		let filename = args[2].clone(); // 기회비용이 있지만, 작은 데이터라 그냥 가져가보기
		// Result의 is_err메소드: 안씀 -> 에러-> True, 씀 -> err아님 ->False
		// env::var 환경변수
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		
		Ok(Config { query, filename, case_sensitive})*/
		println!("{}",args.len());  // 3 : next를 쓰면 개수가 줄어들어서?

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a qeury string"),
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name")
		};

		let case_senistive_args_option = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a case_sensitive option")
		};

		let case_sensitive_args = if case_senistive_args_option == String::from("true") {
			true
		} else if case_senistive_args_option == String::from("fasle") {
			false
		} else {
			return Err("you can input only between true or false for case_sensitive_args")
		};

		let case_sensitive_env = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive_env, case_sensitive_args} )
		

	}
}


// run 함수 추출하기부터 하기

// Box<Error> Box<>는 특성 오브젝트 (17장)
pub fn run(config: Config) -> Result<(), Box<Error>> {
	let mut f = File::open(config.filename)?;
	
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	/*
	// case_sensitive: true -> 대소문자구별, false: 모두 소문자로 바꿔서 상관없게 만듦
	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line)
	}
	
	Ok(())
	*/

	// 과제: 커맨드라인 인수를 우선순위 -> and이어야지 false (args == true -> 무조건 true)

	let mut case_sensitive;
	if config.case_sensitive_args && config.case_sensitive_env == false {
		case_sensitive = false;
	} else {
		case_sensitive = true;
	}

	let results = if case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};

	for line in results {
		println!("{}", line)
	}
	
	Ok(())

}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
	/*
	let mut results = Vec::new();
	
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line)
		}
	}
	
	results
	*/
	// 반복자 어댑터로 변경

	contents.lines()
		.filter(|line| line.contains(query))
		.collect()

}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) ->Vec<&'a str> {
	
	let query = query.to_lowercase();  // 이를 이용해 대소문자 관계 없이 만듦, to_lowercase는 새로운 데이터를 생성함 -> String 반환
	/*
	let mut results = Vec::new();

	for line in contents.lines() {
		// query는 String상태이기에, 문자열slice를 받는 contains() 메소드를 위해 & 추가 -> &query
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	results
	*/

	// 반복자 사용
	contents.lines()
		.filter(|x| x.to_lowercase().contains(&query))
		.collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}


	











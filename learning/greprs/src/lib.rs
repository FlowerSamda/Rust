// 기존 함수, 구조체들을 크레이트화!

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
	pub query: String,
	pub filename: String,
} 

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		
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

// Box<Error> Box<>는 특성 오브젝트 (17장)
pub fn run(config: Config) -> Result<(), Box<Error>> {
	let mut f = File::open(config.filename)?;
	
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;
	
	for line in search(&config.query, &contents) {
		println!("{}", line)
	}
	
	Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line)
		}
	}
	
	results
}


#[cfg(test)]
mod test {
	use super::*;
	
	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
		Rust:
		safe, fast, productive.
		pick three.";
		
		assert_eq!(
		vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
}











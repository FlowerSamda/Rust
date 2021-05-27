use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("올바르지 않은 입력입니다.");

    println!("입력 값: {}", guess);
    

}
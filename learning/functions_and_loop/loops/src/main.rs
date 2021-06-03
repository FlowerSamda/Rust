fn main() {
    let mut counter = 0;

    let _result = loop {
        counter += 1;
        
        if counter == 100 {
            break counter *2;
        }

    };

    println!("result: {}", _result);
    let mut number = 3;

    while number != 0 {
        println!("number: {}", number);
        number -= 1;
    }
    println!("발사!");

    let faren = c_to_f_inverter(36.0);
    println!("celcius 36.0: farenheit {}.", faren);
}


//과제 - 섭씨 -> 화씨 온도 변환기

fn c_to_f_inverter(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}0

fn main() {

    let mut input1 = String::from("Add Sally to Engineering");
    let mut input2 = String::from("Add DK to Engineering");
    let mut input3 = String::from("Add Bruce to Sales");
    let mut input4 = String::from("Add HK to Engineering");

    enum Sector {
        Engineering(String),
        Sales(String),
    }

    let parsed_string1 = parse_string(input1);
    let parsed_string2 = parse_string(input2);
    let parsed_string3 = parse_string(input3);
    let parsed_string4 = parse_string(input4);

    println!("{:?}, {:?}, {:?}, {:?}", parsed_string1, parsed_string2, parsed_string3, parsed_string4);

    let employees = vec![parsed_string1, parsed_string2, parsed_string3, parsed_string4];

    let mut engineering_sec = vec![];
    let mut sales_sec = vec![];

    for employee in employees {
        if &employee.1 == "Engineering" {
            engineering_sec.push(employee)
        } else {
            sales_sec.push(employee)
        }
    }

    engineering_sec.sort();
    sales_sec.sort();

    println!("engineering_sec: {:?}", engineering_sec);
    println!("sales_sec: {:?}", sales_sec);

}

fn parse_string(mut string_data: String) -> (String, String) {

    string_data.push(' ');  // 마지막에 공백을 하나 줌(계산 위해)

    let mut space_index = Vec::new();
    // space를 반환
    for (i, c) in string_data.chars().enumerate() {
        if c == ' ' {
            space_index.push(i);
        }
    } 

    // word 찾기
    let mut words_vec = Vec::new();
    let mut pointer = 0;
    for i in space_index {
        words_vec.push(&string_data[pointer..i]);
        pointer = i + 1;
    }
    
    // 유의미한 튜플 반환
    let name = String::from(words_vec[1]);
    let sector = String::from(words_vec[3]);

    (name, sector)  // ("DK", "Sales") 이런 식!
}

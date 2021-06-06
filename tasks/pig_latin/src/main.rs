fn main() {
    

    let data = "I need a help";

    let mut raw_string = data.to_string();  // 영어는 각 단어가 1바이트씩
    raw_string.push(' '); // 마지막에 공백을 줘서 words 반환 때 사용
    let _string_len = raw_string.len();
    println!("len: {}", raw_string.len());

    let mut words = Vec::new();
    
    // 공백 인덱스 찾기
    let space_index = find_space(&raw_string);

    //words vector 반환
    // 나중에 이걸 뺴려고 할 때, 오류가 생기는데 한번 알아보자!
    let mut pointer = 0;
    for i in space_index {
        let word = &raw_string[pointer..i];
        words.push(word);
        pointer = i + 1
    }


    pig_latin_translator(&words);
    println!("{:?}", words);



}

// 공백 인덱스 찾기
fn find_space(raw_string: &String) -> Vec<usize> {

    let mut space_index = Vec::new();

    for (i, c) in raw_string.chars().enumerate() {
        if c == ' ' {
            space_index.push(i);
        }
    } 

    space_index
}



fn pig_latin_translator(words_vec: &Vec<&str>) {

    let mut pig_latin_vec = vec![];
    let ay = "ay";

    for word in words_vec {
        println!("{}", word);
        
        let pig_word = format!("{}{}{}", &word[1..], &word[0..1], ay);
    


        pig_latin_vec.push(pig_word);
    }
    println!("{:?}", pig_latin_vec)

    
}




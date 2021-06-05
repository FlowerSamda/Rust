use std::collections::HashMap;

fn main() {
    
    let mut nums_vector = Vec::new();

    nums_vector.push(1);
    nums_vector.push(3);
    nums_vector.push(7);
    nums_vector.push(9);
    nums_vector.push(11);
    nums_vector.push(13);
    nums_vector.push(15);
    nums_vector.push(17);
    nums_vector.push(3);

    let mean = sum(&nums_vector);
    println!("mean: {}", mean);
    println!("nums_vector: {:?}", nums_vector);  // 소유권 체크

    let median = median(&nums_vector);
    println!("median: {}", median);
    println!("nums_vector: {:?}", nums_vector);  // 소유권 체크

    let mode = mode(&nums_vector);
    println!("mode: {:?}", mode);
    println!("nums_vector: {:?}", nums_vector);  // 소유권 체크
}


// calculate sum
fn sum(nums_vector:&Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for i in nums_vector {
        sum += i
    }
    
    let mean = (sum as f64) / (nums_vector.len() as f64);
    mean
}

// return median
fn median(nums_vector: &Vec<i32>) -> i32 {
    let len = nums_vector.len();
    
    if len % 2 == 0 {
        let index1 = &len / 2;
        let index2 = &len / 2 - 1;

        let num1: &i32 = &nums_vector[index1];
        let num2: &i32 = &nums_vector[index2];

        let median = (num1 + num2) / 2 ;
        median

        

    } else {
        let index = (&len - 1) / 2;
        let median = &nums_vector[index];

        *median  // 이렇게 쓰게 시킴...
    }
}

// 키: &Vec<i32>의 num(&i32), 값: count(&mut i32 이 부분은 더 알아보기)
fn mode (nums_vector: &Vec<i32>) ->HashMap<&i32, i32> {
    let mut mode_map = HashMap::new();

    for num in nums_vector {
        let count = mode_map.entry(num).or_insert(0);
        *count +=1;
        }    

    mode_map  // 여기서 가장 값이 큰 걸 찾는 메소드로 반환
}



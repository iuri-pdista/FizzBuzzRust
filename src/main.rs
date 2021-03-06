fn main() {
    let boxed_vector: Box<Vec<u32>> = Box::new(vec![0; 100]);
    let numbers_1_to_100: Vec<u32> = increment_all_elements(boxed_vector);
    for x in numbers_1_to_100 {
        fizz_buzz(x);
    }
}

fn increment_all_elements (elements: Box<Vec<u32>>)-> Vec<u32>{
    let mut count = 0;
    elements.into_iter().map(|x| {count+= 1; x + count}).collect()
}

fn fizz_buzz( num:u32 ){
    if num == 0{
        println!("{}\n", num);
    }
    else if is_fizz_buzz(num){
        println!("FizzBuzz\n");
    }
    else if is_fizz(num){
        println!("Fizz\n");
    }
    else if is_buzz(num){
        println!("Buzz\n");
    }
    else{
        println!("{}\n",num);
    }
}

fn is_fizz_buzz( num: u32 ) -> bool{
    if is_fizz(num) && is_buzz(num){
        true
    }
    else{
        false
    }
}

fn is_fizz( num: u32 )->bool{
    if num % 3 == 0{
        true
    }else{
        false
    }
}

fn is_buzz( num: u32 ) -> bool{
    if num % 5 == 0{
        true
    }
    else{
        false
    }
}

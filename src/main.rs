fn main() {
    for x in 0..100{
        fizz_buzz(x);
    }
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

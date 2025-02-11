fn main() {
    let ans = is_even(15);
    let fibNumber = fib(6);
    println!("{}",fibNumber);
}



fn is_even(num:i32) ->bool { 



    if num % 2 ==0 {
        return  true;
    }
    else {
        return  false;
    }

}
// 0 1 1 2  3  5 
// solving the fibo serires
fn fib(num:i64) -> u64{
     let  mut first = 0;
     let  mut second = 1;
   
    if num == 0 {
        return first
    }

    if num == 1 {
        return  second;
    }

    for _   in 1..num -1 {
        let  temp = second;
        second = second + first;
        first = temp;


        
    }
    return  second;

}
fn main(){
    println!("{}", fib(10));
}

fn fib( n: u32)-> u32{
    let mut a = 0;
    let mut b = 1;
    if n == 0 {
        return a;
    }

    if n == 1{
        return b;
    }

    for _ in 0..(n-1){
        let temp = b;
        b = a + b;
        a = temp;
    }

    return b;
}

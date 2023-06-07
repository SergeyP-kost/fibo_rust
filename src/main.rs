use std::io;


fn main() {

    loop {

        let mut int = String::new(); 

        println!("Введите число.");
        
        io::stdin()
            .read_line(&mut int)
            .expect("Не удалось прочитать.");

        let int: u64 = match int.trim()
            .parse() {
                Ok(int) => int,
                Err(_) => continue,
            };

        println!("Значение для элемента {} -> {}", int, fibo(int));

        break;
    }       
}


fn fibo(n: u64) -> u64 {

    let mut fib1: u64 = 1;
    let mut fib2: u64 = 1;

    let mut index = 0;

    while index < (n - 2) {
        let fib_sum: u64 = fib1 + fib2;
        fib1 = fib2;
        fib2 = fib_sum;
        index += 1;
    }

    return fib2
}

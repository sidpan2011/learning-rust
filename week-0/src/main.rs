fn main() {
    println!("{}", fib(9));
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    } else if num == 1 {
        return second;
    }
    for _ in 0..num - 1 {
        let next = first + second;
        first = second;
        second = next;
    }
    second
}
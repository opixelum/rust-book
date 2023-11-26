fn fibonacci(number: u128) -> u128 {
    return
        if number == 0 { 0 }
        else if number == 1 { 1 }
        else { fibonacci(number - 1) + fibonacci(number - 2) }
}

fn main() {
    let mut index: u8 = 0;
    loop {
        println!("fibonacci({}) = {}", index, fibonacci(index as u128));
        index += 1;
        if index == 11 { break } ;
    }
}

fn main() {
    let mut cnt = 0;
    'counting_up: loop {
        println!("count :{}", cnt);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if cnt == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        cnt += 1;
    }
    println!("End count: {}", cnt);
}

fn main() {
    test_function(10);

    let mut x = 3;
    let y = x + 1;

    println!("The value of y is: {}", y);
}

fn test_function(x: i32) {
    println!("The value of x is {}", x);
}

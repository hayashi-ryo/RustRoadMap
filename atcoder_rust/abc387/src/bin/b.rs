use proconio::input;

fn main() {
    input! {
        n: u32,
    };
    let mut result = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j != n {
                result += i * j;
            }
        }
    }

    println!("{}", result);
}

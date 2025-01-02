// IPアドレスの種類列挙
enum IPAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    // 以下のように具体的な構造体を定義しなくても、enumのそれぞれの要素に対して具体的な型を保持することができる。
    let home = IPAddr::V4(String::from("127.0.0.1"));
    let home = IPAddr::V6(String::from("::1"));
}

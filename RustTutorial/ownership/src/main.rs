fn main() {
    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s.clone()); // sの値が関数にムーブされ...
                                // ... ここではもう有効ではない
    let x = 5; // xがスコープに入る
    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
    println!("string s: {}", s);
}

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  //

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // こ

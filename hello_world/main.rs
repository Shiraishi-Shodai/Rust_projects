fn main() {
    // https://doc.rust-lang.org/std/string/struct.String.html#method.as_ptr
    // ポインタは文字列スライスの最初のバイトを表す。
    
    // let a : String = String::from("Hello");
    // println!("variable a: ptr = {:p}, len = {}, capacity = {}", a.as_ptr(), a.len(), a.capacity());

    // let mut b : String = a;
    // println!("variable b: ptr = {:p}, len = {}, capacity = {}", b.as_ptr(), b.len(), b.capacity());

    // b = String::from("konnichiha");
    // println!("variable b: ptr = {:p}, len = {}, capacity = {}", b.as_ptr(), b.len(), b.capacity());
    
    // ダングリングポインタ：他人に渡されてしまった可能性のあるポインタ
    // let reference_to_nothing = dangle();

    let s = String::from("Hello World!");
    let word = first_word(&s);

    // s.clear();
    println!("The first word in {} is {}.", s, word);
}

// fn dangle() -> &String {
//     // sはこの関数で定義されているため、この関数を抜けるとこのデータは消去される。
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str)-> &str{
    
    for (i, j) in s[..].chars().enumerate() {
        if  j == ' ' {
            return &s[..i];
        }
    }

    &s[..]
}
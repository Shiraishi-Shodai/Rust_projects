/*
sliceは一部の参照
*/


fn main() {
    println!("Hello, world!");

    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();
    // println!("{} {}", word, s);

    // let s = String::from("hello world");
    // let hello = &s[..5];
    // let world = &s[6..];

    // // s[..] //コンパイル時に文字列の長さが決まらないためコンパイルエラー
    // println!("{} {} {}", hello, world, &s[..]);

    // let mut s = String::from("hello world");
    // let word = first_word(&s); // wordは不変参照
    // s.clear(); // clear関数の中で可変参照を扱おうとする(wordが存在する間はsの中身が変えられない。つまりwordとsが同期された状態)
    // println!("{}", word);

    // 文字列リテラルは不変参照
    // let s: &str = "Hello, world"; 
    // println!("{}", s);


    let my_string = String::from("hello world!");
    let word = first_word(&my_string[..]);
    println!("{}", word);
    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    let word = first_word(my_string_literal);
    println!("{}", word);

}

// fn first_word(s: &String)-> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         // itemを条件式を見て、自動的に参照から値に変換してくれる
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &String)-> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         // itemを条件式を見て、自動的に参照から値に変換してくれる
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn first_word(s: &str)-> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // itemを条件式を見て、自動的に参照から値に変換してくれる
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


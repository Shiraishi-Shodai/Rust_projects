/*
関数に値は渡したいけど、所有権は渡したくない場合はどうすればいい？　＝＞ 参照を使う
*/

fn main() {
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);
    // //  '{}'の長さは、{}です。
    // println!("The length of {} is {}", s1, len);

    let mut s1 = String::from("hello");
    // change(&s1);　// Error
    change(&mut s1); // OK
    println!("{}", s1);

    // 可変参照は式を抜けるときにdropされるためOK
    // {
    //     let r1 = &mut s1;
    // }

    // let r2 = &mut s1;

    /*不変参照は複数定義してもOK(可変参照と不変参照は同時に定義できない)*/
    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1;

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

// &をつけることで、所有権を渡さずに関数の引数として受け取る(借用)
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 参照も不変
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 可変の参照を渡すと値を変えられる(可変な参照は１つしか作れない)
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// ダングリング参照(dangle関数の中でしか扱えないsをmain関数の中に返すことはできない)
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

// 所有権をムーブする OK
fn dangle() -> String {
    let s = String::from("hello");
    s
}

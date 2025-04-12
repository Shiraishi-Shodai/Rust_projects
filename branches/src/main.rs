/*
アーム：条件式とその条件に一致したときの処理をまとめたもの
論理型：論理値以外の値が自動的に論理値に変換されることはない
if式で評価した値を返すときは、ifとelseで返す値の型を揃える必要がある。（ifで返す値の型が基準となる）
*/

fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("contidtion was false");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is {}", number);

    let number = if condition {"a"} else {"six"}; // Error

}

/*
所有権はヒープを管理するために存在する。（ヒープ管理者券）

Rustの各値は、所有者と呼ばれる変数と対応している。
いかなる時も所有者は一つである。
所有者がスコープから外れたら、値は破棄される。

スコープ：要素が有効になる範囲

メモリは、実行時にOSに要求される。
String型を使用し終わったら、OSにこのメモリを返還する方法が必要である。(String型はサイズが可変のため、ヒープに保存される)


Rustはメモリを所有している変数がスコープを抜けたら、自動的にdropと呼ばれる関数を呼び出し、メモリは自動的に返還される
*/

fn main() {
    println!("Hello, world!");

    // let mut s = String::from("hello");
    // s.push_str(", world");
    // println!("{}", s);


    /*ムーブ */
    /*
    String型の部品(これはスタックに保存される)
    文字列の中身を保持するメモリへのポインタと長さ、許容量
    */
    // let s1 = String::from("hello");
    // let s2 = s1; // s1のString型のデータ(スタックにあるポインタ、長さ、許容量)がs2にコピーされる。（ポインタが示すヒープ上のデータはコピーしない)

    /*
     普通に考えると、main関数を抜けるとs1とs2が互いに保持しているポインタが示すメモリ(ヒープ)を開放してしまう。[二重開放]が起こる
    しかし、二重降下を防ぐためにs2にs1をコピーした時点でs1は無効化される。  
     */

    //  println!("{}, world!", s1); // Error
    //  println!("{}, world!", s2); // OK

     
     /*クローン*/
    //  let s1 = String::from("hello");
    //  let s2 = s1.clone();
    //  println!("{}, world!", s1); // OK
    //  println!("{}, world!", s2); // OK

    /*スタックのみのデータコピー*/
    /*なぜ以下のコードでは、xは無効化されないの？
    コンパイルの時点で、基地のデータ型の値はスタックに保持されるため、実際の値をコピーするのも高速だから
    */
    // let x = 5;
    // let y = x;
    // println!("x = {}, y = {}", x , y);


    /*関数と所有権*/
    let s = String::from("hello"); // sがスコープに入る
    takes_ownership(s);//sの値が関数にムーブされる
    //ここではもうsは無効

    let x = 5; // xがスコープに入る
    makes_copy(x); // xも関数にムーブされる
    // i32はCopyなので、この後にxを使っても大丈夫

    println!("s = {}", s); // Error
    println!("x = {}", x); // OK

}

fn takes_ownership(some_string: String) {// some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、dropを呼び出す。後ろ盾していたメモリが開放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integereがスコープを抜ける。何も特別なことは起きない。

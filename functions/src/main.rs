// 式指向型言語
// 文：何らかのどうさをして値を返さない命令 例：関数も文
// 式：評価されるもの　例：{}, 4

fn main() {
    println!("Hello, world!");
    // another_function(5, 'h');

    // let y = 6; // 文
    // let x = let y = 6; // 文は値を返さないためエラー

    let y = {
        let x = 3;
        x + 1 // 式はセミコロンをつけない。つけると文に変わってしまう

    };

    // println!("The value of y is {}", y);

    let x = five();
    println!("The value of x is {}", x);
}

fn another_function(x: i32, unit_label: char) {
    println!("another function acitivate");
    println!("The value of x is : {} {}", x, unit_label);
}

fn five() -> i32 {
    5 //セミコロンをつけると、文に変換し値としてい評価されなくなる。つまり、返り値が空になる
}
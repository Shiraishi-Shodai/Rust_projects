/*
ループラベル：breakやcontinueが適用されるループを指定するもの
*/

fn main() {
    println!("Hello, world!");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         } 
    
    //         if count == 2 {
    //             break 'counting_up; // counting_upのloopを抜ける
    //         }
    
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }

    // println!("End count = {}", count);


    // let mut number = 2;
    // while number != 0 {
    //     println!("The number is {}", number);
    //     number -=1;
    // }


    let a = [10, 20, 30, 40, 50];
    // let a = [3; 5];

    // for element in a {
    //     println!("element is {}", element);
    // }

    for element in (1..=4).rev() {
        println!("element is {}", element);
    }

}

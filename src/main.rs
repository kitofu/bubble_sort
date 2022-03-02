// バブルソートアルゴリズムを Rust で実装してください。
// 仕様
// -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
// 提出方法
// GitHub にて公開し、このポストにリンクをリプライする。
// リポジトリの運用方法は自由。
// 提出期限
// 2022/03/04（金）12:00 まで


use std::io;
use std::io::{stdout, Write};
use rand::Rng;



fn main() {
    println!("This is bubble sort!");

    let mut a: [i32; 101] = [1; 101];

    for n in 1..101{
        let rand = rand::thread_rng().gen_range(-1000, 1001);
        a[n] = rand;
        println!("n{} = {}", n, a[n]);
    }


}

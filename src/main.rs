// バブルソートアルゴリズムを Rust で実装してください。
// 仕様
// -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
// 提出方法
// GitHub にて公開し、このポストにリンクをリプライする。
// リポジトリの運用方法は自由。
// 提出期限
// 2022/03/04（金）12:00 まで


use rand::Rng;



fn main() {
    println!("This is bubble sort!");

    //配列を用意するi
    const ARRAY_NUM:usize = 100;
    let mut a: [i32; ARRAY_NUM] = [1; ARRAY_NUM];

    for n in 0..ARRAY_NUM{
        let rand = rand::thread_rng().gen_range(-1000, 1001);
        a[n] = rand;
        println!("n{} = {}", n, a[n]);
    }
    //アルゴリズム実行する
    for i in 0..ARRAY_NUM - 1{
        for j in 0..ARRAY_NUM-1-i{
            if a[j] > a[j+1] {
                let big_num = a[j];
                a[j] = a[j+1];
                a[j+1] = big_num;
            }
        }
    }
    //Sort shiowatta
    for n in 0..ARRAY_NUM{
            println!("Sort shimashita a_{} = {}", n, a[n]);
        }

}

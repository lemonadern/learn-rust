#![allow(unused)]

// 整数のリストが与えられ、ベクタを使ってmean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。

use std::collections::HashMap;

fn main() {
    let v = vec![1, 3, 9, 2, 7, 3, 5, 6, 7, 8, 8, 3, 8, 9];

    println!("{}", mean(&v));
    println!("{}", mode(&v));
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    for i in v {
        sum += f64::from(*i);
    }
    sum / (v.len() as f64)
}

fn median() {}

fn mode(v: &Vec<i32>) -> i32 {
    let mut hash_map = HashMap::new();

    for i in v {
        let c = hash_map.entry(i).or_insert(0);
        *c += 1;
    }

    if let Some(i) = hash_map.values().max() {
        i32::from(*i)
    } else {
        -1
    }
}

// 文字列をピッグ・ラテン(訳注: 英語の言葉遊びの一つ)に変換してください。
// 各単語の最初の子音は、 単語の終端に移り、"ay"が足されます。従って、"first"は"irst-fay"になります。
// ただし、 母音で始まる単語には、お尻に"hay"が付け足されます("apple"は"apple-hay"になります)。
// UTF-8エンコードに関する詳細を心に留めておいてください！

fn main() {
    let s1=String::from("first");
    let s2 =String::from("apple");
    println!("Hello, world!");
}

// fn pigganize(s: &String)-> String{}
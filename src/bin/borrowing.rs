fn main() {
    // "Hello, World"はそもそもstringじゃないの？
    // https://qiita.com/yagince/items/e7474839246ced595f7a
    // https://matsumaee.hatenablog.com/entry/2021/07/19/194550
    // まず"Hello, World"はStringとして可変長で生成される。それをto_string()することで
    // &strとなり、文字列スライスになるってコト!?
    let important_data = "Hello, World".to_string();
    calc_data(&important_data);
    println!("{}", important_data)
}

fn calc_data(data: &String) {
    println!("{}", data);
}

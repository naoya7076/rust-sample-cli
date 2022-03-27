use std::fmt;

impl fmt::Display for MyError {
    // 'とは何だ。static？
    // ライフタイムの文脈で使われている
    // https://doc.rust-jp.rs/rust-nomicon-ja/lifetimes.html
    // このブログが参考になりそう
    // https://numb86-tech.hatenablog.com/entry/2021/05/22/195352
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";

    // ?はOK(t)ならtを返してErr(e)なら早期リターン
    let num_str = std::fs::read_to_string(path).map_err(MyError::Io)?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(MyError::Num)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}

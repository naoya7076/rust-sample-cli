use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    // ?はOK(t)ならtを返してErr(e)なら早期リターン
    // with_contextに「文字列を返すクロージャを渡す」とあるがクロージャがわからんじゃ
    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read string from {}", path))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .context("failed to parsee string")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}

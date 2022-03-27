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
        Err(e) => match e {
            MyError::Io(cause) => println!("I/O Error: {}", cause),
            MyError::Num(cause) => println!("Parse Error: {}", cause),
        },
    }
}

fn some_method<'a>(arg1: &'a str, _arg2: &str) -> &'a str {
    arg1
}

fn main() {
    let z;
    let x = "foo".to_string();
    {
        let y = "bar".to_string();
        z = some_method(&x, &y)
    }
    println!("{}", z);
}

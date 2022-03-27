fn some_method<'a>(arg1: &'a String, arg2: &String) -> &'a String {
    arg1
}

fn main() {
    let arg1 = "hoge".to_string();
    let arg2 = "piyo".to_string();
    let returned_arg1 = some_method(&arg1, &arg2);
}
